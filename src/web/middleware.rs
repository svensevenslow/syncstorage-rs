//! # Web Middleware
//!
//! Matches the [Sync Storage middleware](https://github.com/mozilla-services/server-syncstorage/blob/master/syncstorage/tweens.py) (tweens).
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{
    http::{header, HeaderName, HeaderValue, Method, StatusCode},
    Error, FromRequest, HttpMessage, HttpRequest, HttpResponse, Result,
};
use futures::{
    future::{self, Either},
    Future, IntoFuture,
};

use db::{params, util::SyncTimestamp, Db};
use error::{ApiError, ApiErrorKind};
use server::ServerState;
use web::extractors::{BsoParam, CollectionParam, HawkIdentifier, PreConditionHeader, PreConditionHeaderOpt};

/// Default Timestamp used for WeaveTimestamp middleware.
#[derive(Default)]
struct DefaultWeaveTimestamp(SyncTimestamp);

/// Middleware to set the X-Weave-Timestamp header on all responses.
pub fn add_weave_timestamp<S, B>(
    req: ServiceRequest,
    srv: &mut S,
) -> impl IntoFuture<Item = ServiceResponse<B>, Error = Error>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    srv.call(req).map(|mut resp| {
        let ts = match req.extensions().get::<DefaultWeaveTimestamp>() {
            Some(ts) => ts.0.as_seconds(),
            None => return resp,
        };

        let weave_ts = if let Some(val) = resp.headers().get("X-Last-Modified") {
            let resp_ts = val
                .to_str()
                .map_err(|e| {
                    let error: ApiError = ApiErrorKind::Internal(format!(
                        "Invalid X-Last-Modified response header: {}",
                        e
                    ))
                    .into();
                    error
                })
                .unwrap()
                .parse::<f64>()
                .map_err(|e| {
                    let error: ApiError = ApiErrorKind::Internal(format!(
                        "Invalid X-Last-Modified response header: {}",
                        e
                    ))
                    .into();
                    error
                })
                .unwrap();
            if resp_ts > ts {
                resp_ts
            } else {
                ts
            }
        } else {
            ts
        };
        resp.headers_mut().insert(
            header::HeaderName::from_static("x-weave-timestamp"),
            header::HeaderValue::from_str(&format!("{:.*}", 2, &weave_ts))
                .map_err(|e| {
                    let error: ApiError = ApiErrorKind::Internal(format!(
                        "Invalid X-Weave-Timestamp response header: {}",
                        e
                    ))
                    .into();
                    error
                })
                .unwrap(),
        );
        resp
    })
}

/// Middelware to initialize the db and enforce transactions
pub fn db_transaction<S, B>(
    s_req: ServiceRequest,
    srv: &mut S,
) -> impl IntoFuture<Item = ServiceResponse<B>, Error = Error>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    let (req, payload) = s_req.into_parts();
    let collection = CollectionParam::from_request(&req, &mut payload)
        .map(|param| param.collection.clone())
        .ok();
    let user_id = HawkIdentifier::from_request(&req, &mut payload).unwrap();
    let in_transaction = collection.is_some();

    let fut = req
        .app_data::<ServerState>()
        .unwrap()
        .db_pool
        .get()
        .and_then(move |db| {
            let db2 = db.clone();
            let fut = if let Some(collection) = collection {
                // Take a read or write lock depending on request method
                let lc = params::LockCollection {
                    user_id,
                    collection,
                };
                Either::A(
                    match *req.method() {
                        Method::GET | Method::HEAD => db.lock_for_read(lc),
                        _ => db.lock_for_write(lc),
                    }
                    .or_else(move |e| {
                        // Middleware::response won't be called: rollback immediately
                        db2.rollback().and_then(|_| future::err(e))
                    }),
                )
            } else {
                // If we're not operating on a collection, don't take a lock
                Either::B(future::ok(()))
            };
            fut.and_then(move |_| {
                // track whether a transaction was started above via the
                // lock methods
                req.extensions_mut().insert((db, in_transaction));
                future::ok(None)
            })
        })
        .map_err(Into::into);
    srv.call(s_req).map(|mut s_resp| {
        if let Some((db, in_transaction)) =
            s_resp.request().extensions().get::<(Box<dyn Db>, bool)>()
        {
            if *in_transaction {
                let resp = s_resp.response_mut();
                let fut = match resp.error() {
                    None => db.commit(),
                    Some(_) => db.rollback(),
                };
                let fut = fut.and_then(|_| Ok(resp)).map_err(Into::into);
                return s_resp;
            }
        }
        s_resp
    })
}

/// The resource in question's Timestamp
pub struct ResourceTimestamp(SyncTimestamp);

#[derive(Debug)]
pub struct PreConditionCheck;

pub fn precondition_check<S, B> (
    s_req: ServiceRequest,
    srv: &mut S
) -> impl IntoFuture<Item = ServiceResponse<B>, Error=Error>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error=Error>,
    {

    // pre-call
    // This middleware must be wrapped by the `DbTransaction` middleware to ensure a Db object
    // is available.
    let (req, payload) = s_req.into_parts();

    // Can't use From complains about Option not in scope, so semi-hack.
    let precondition =
        match PreConditionHeaderOpt as FromRequest<ServerState>>::from_request(&req, &payload)
        {
            Ok(precondition) => {
                match precondition.opt {
                    Some(precondition) => precondition,
                    // TODO: Is this correct?
                    None => return HttpResponse::Ok()
                }
            },
            // TODO: Return a Server Error?
            Err(e) => return e.into(),
        };
    let user_id = HawkIdentifier::from_request(&req, &mut payload)?;
    let db = <Box<dyn Db>>::from_request(&req, &mut payload)?;
    let collection = CollectionParam::from_request(&req, &mut payload)
        .ok()
        .map(|v| v.collection);
    let bso = BsoParam::from_request(&req, &mut payload.map(|v| v.bso));
    let req = req.clone(); // Clone for the move to set the timestamp we get
    let fut = db.unwrap()
        .extract_resource(user_id, collection, bso)
        .and_then(move |resource_ts: SyncTimestamp| {
            // Ensure we stash the extracted resource timestamp on the request in case its
            // requested elsewhere
            req.extensions_mut().insert(ResourceTimestamp(resource_ts));
            let status = match precondition {
                PreConditionHeader::IfModifiedSince(header_ts) if resource_ts <= header_ts => {
                    StatusCode::NOT_MODIFIED
                }
                PreConditionHeader::IfUnmodifiedSince(header_ts) if resource_ts > header_ts => {
                    StatusCode::PRECONDITION_FAILED
                }
                _ => return future::ok(None),
            };
            let resp = HttpResponse::build(status)
                .header("X-Last-Modified", resource_ts.as_header())
                .body(""); // 304 can't return any content
            future::ok(Some(resp))
        })
        .map_err(Into::into);

    // Make the call, and do the post.
    srv.call(s_req).map(|mut s_resp| {

        // Ensure all outgoing requests from here have a X-Last-Modified
        if s_resp.headers().contains_key("X-Last-Modified") {
            return Ok(Response::Done(resp));
        }

        // See if we already extracted one and use that if possible
        if let Some(resource_ts) = s_req.extensions().get::<ResourceTimestamp>() {
            let ts = resource_ts.0;
            if let Ok(ts_header) = header::HeaderValue::from_str(&ts.as_header()) {
                resp.headers_mut().insert("X-Last-Modified", ts_header);
            }
            return Ok(Response::Done(resp));
        }

        // Do the work needed to generate a timestamp otherwise
        let user_id = HawkIdentifier::from_request(&s_req, &())?;
        let db = <Box<dyn Db>>::from_request(&s_req, &())?;
        let collection = CollectionParam::from_request(&s_req, &())
            .ok()
            .map(|v| v.collection);
        let bso = BsoParam::from_request(&s_req, &()).ok().map(|v| v.bso);
        let fut = db
            .extract_resource(user_id, collection, bso)
            .and_then(move |resource_ts: SyncTimestamp| {
                if let Ok(ts_header) = header::HeaderValue::from_str(&resource_ts.as_header()) {
                    resp.headers_mut().insert("X-Last-Modified", ts_header);
                }
                future::ok(s_resp)
            })
            .map_err(Into::into);
        Response::Future(Box::new(fut))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;
    use actix_web::test::TestRequest;
    use chrono::Utc;

    #[test]
    fn test_no_modified_header() {
        let weave_timestamp = WeaveTimestamp {};
        let req = TestRequest::default().finish();
        let resp = HttpResponse::build(http::StatusCode::OK).finish();
        match weave_timestamp.start(&req) {
            Ok(Started::Done) => (),
            _ => panic!(),
        };
        let resp = match weave_timestamp.response(&req, resp) {
            Ok(Response::Done(resp)) => resp,
            _ => panic!(),
        };
        let weave_hdr = resp
            .headers()
            .get("X-Weave-Timestamp")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let weave_hdr = (weave_hdr * 1000.0) as u64;
        // Add 10 to compensate for how fast Rust can run these
        // tests (Due to 2-digit rounding for the sync ts).
        let ts = (Utc::now().timestamp_millis() as u64) + 10;
        assert_eq!(weave_hdr < ts, true);
        let ts = ts - 2000;
        assert_eq!(weave_hdr > ts, true);
    }

    #[test]
    fn test_older_timestamp() {
        let weave_timestamp = WeaveTimestamp {};
        let ts = (Utc::now().timestamp_millis() as u64) - 1000;
        let hts = format!("{:.*}", 2, ts as f64 / 1_000.0);
        let req = TestRequest::default().finish();
        let resp = HttpResponse::build(http::StatusCode::OK)
            .header("X-Last-Modified", hts.clone())
            .finish();
        match weave_timestamp.start(&req) {
            Ok(Started::Done) => (),
            _ => panic!(),
        };
        let resp = match weave_timestamp.response(&req, resp) {
            Ok(Response::Done(resp)) => resp,
            _ => panic!(),
        };
        let weave_hdr = resp
            .headers()
            .get("X-Weave-Timestamp")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let hts = hts.parse::<f64>().unwrap();
        assert!(weave_hdr > hts);
    }

    #[test]
    fn test_newer_timestamp() {
        let weave_timestamp = WeaveTimestamp {};
        let ts = (Utc::now().timestamp_millis() as u64) + 4000;
        let hts = format!("{:.*}", 2, ts as f64 / 1_000.0);
        let req = TestRequest::default().finish();
        let resp = HttpResponse::build(http::StatusCode::OK)
            .header("X-Last-Modified", hts.clone())
            .finish();
        match weave_timestamp.start(&req) {
            Ok(Started::Done) => (),
            _ => panic!(),
        };
        let resp = match weave_timestamp.response(&req, resp) {
            Ok(Response::Done(resp)) => resp,
            _ => panic!(),
        };
        let weave_hdr = resp
            .headers()
            .get("X-Weave-Timestamp")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(weave_hdr, hts);
    }
}
