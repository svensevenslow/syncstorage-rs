//! # Web Middleware
//!
//! Matches the [Sync Storage middleware](https://github.com/mozilla-services/server-syncstorage/blob/master/syncstorage/tweens.py) (tweens).
use actix_web::{
    http::{header, Method, StatusCode},
    FromRequest, HttpResponse,
    Error, web::Data,
};

//use actix_web::dev::{Body, MessageBody, ServiceRequest, ServiceResponse};
use actix_web::dev::{MessageBody, ServiceRequest, ServiceResponse};
//use actix_http::body::Body;

use actix_service::{Service, Transform};

use futures::{
    future::{self, FutureResult,Either},
    Future,
    Poll
};

use crate::db::{params, util::SyncTimestamp, Db};
use crate::error::{ApiError, ApiErrorKind};
use crate::server::ServerState;
use crate::web::extractors::{BsoParam, CollectionParam, HawkIdentifier, PreConditionHeader, PreConditionHeaderOpt};

/// Default Timestamp used for WeaveTimestamp middleware.
#[derive(Default)]
struct DefaultWeaveTimestamp(SyncTimestamp);

pub struct WeaveTimestampMiddleware<S> {
    service: S,
}

impl<S, B> Service for WeaveTimestampMiddleware<S>
where
B: MessageBody,
S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error= Error>,
S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    // call super poll_ready()
    fn poll_ready(&mut self) -> Poll<(), Self::Error>{
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let ts = DefaultWeaveTimestamp::default();
        Box::new(self.service.call(req).map(move |mut resp| {
            //let (req, _) = req.into_parts();
            //if let Some(ts) = req.extensions().get::<DefaultWeaveTimestamp>() {
            if true {
                let weave_ts = if let Some(val) = resp.headers().get("X-Last-Modified") {
                    let resp_ts = val
                        .to_str()
                        .map_err(|e| -> ApiError { ApiErrorKind::Internal(format!("Invalid X-Last-Modfied response header: {}", e)).into() })
                        .unwrap()
                        .parse::<f64>()
                        .map_err(|e| -> ApiError { ApiErrorKind::Internal(format!("Invalid X-Last-Modified response header: {}", e)).into() })
                        .unwrap();
                    if resp_ts > ts.0.into() {
                        resp_ts
                    } else {
                        ts.0.into()
                    }
                } else {
                    ts.0.into()
                };
                resp.headers_mut().insert(
                    header::HeaderName::from_static("x-weave-timestamp"),
                    header::HeaderValue::from_str(&format!("{:.*}", 2, &weave_ts))
                    .map_err(|e| -> ApiError { ApiErrorKind::Internal(format!("Invalid X-Weave-Timestamp response header: {}", e)).into()})
                    .unwrap()
                )
            };
            resp
        }))
    }
}
/// Middleware to set the X-Weave-Timestamp header on all responses.
pub struct WeaveTimestamp;

impl WeaveTimestamp {
    pub fn new() -> Self {
        WeaveTimestamp::default()
    }
}

impl Default for WeaveTimestamp {
    fn default() -> Self {
        Self
    }
}

impl<S, B> Transform<S> for WeaveTimestamp
where
S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error= Error>,
S::Future: 'static,
B: MessageBody,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError=();
    type Transform = WeaveTimestampMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;
    //type Transform = WeaveTimestampMiddleware<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(WeaveTimestampMiddleware{
            service
        })
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct DbTransactionMiddleware<S>{
//    service: S,
    service: Rc<RefCell<S>>,
}

impl<S,B> Service for DbTransactionMiddleware<S>
where
//B:MessageBody,
S:Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
B: 'static
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        //self.service.poll_ready()
        self.service.borrow_mut().poll_ready()
    }

    fn call(&mut self, sreq: ServiceRequest) -> Self::Future {
        let data:Data<ServerState> = sreq.app_data().unwrap();
        let (req, mut payload) = sreq.into_parts();
        let req = req.clone();
        let collection = CollectionParam::from_request(&req, &mut payload)
            .map(|param| param.collection.clone())
            .ok();
        let user_id = HawkIdentifier::from_request(&req, &mut payload).unwrap();
        let in_transaction = collection.is_some();

        let mut srv = self.service.clone();

        Box::new(data.db_pool.get().and_then(move |db| {
            let db2 = db.clone();
            let fut = if let Some(collection) = collection {
                let lc = params::LockCollection{
                    user_id,
                    collection,
                };
                Either::A(
                    match *req.method() {
                        Method::GET | Method::HEAD => db.lock_for_read(lc),
                        _ => db.lock_for_write(lc)
                    }
                    .or_else(move|e| {
                        db2.rollback().and_then(|_| future::err(e))
                    })
                )
            } else {
                Either::B(future::ok(()))
            };
            fut.and_then(move |_| {
                // track whether a transaction was started above via the
                // lock methods
                req.extensions_mut().insert((db.clone(), in_transaction));
                future::ok(db)
                //future::ok(None)
            })
        }).map_err(Into::into)
            .and_then(move |db| {
            srv.call(sreq).and_then(move |sresp| {
                //if let Some((db, in_transaction)) = sresp.request().extensions().get::<(Box<dyn Db>, bool)>() {
                if true {
                if in_transaction {
                    return Either::A(match sresp.response().error() {
                        None => db.commit(),
                        Some(_) => db.rollback(),
                    }.map_err(Into::into).and_then(|_| future::ok(sresp)));
                }
            }
                Either::B(future::ok(sresp))
                //sresp
            })
        }))
    }
}


pub struct DbTransaction;

impl DbTransaction {
    pub fn new() -> Self {
        DbTransaction::default()
    }
}

impl Default for DbTransaction {
    fn default() -> Self {
        Self
    }
}


impl<S, B> Transform<S> for DbTransaction
where
//B: MessageBody,
S: Service<Request =ServiceRequest, Response=ServiceResponse<B>, Error= Error> + 'static,
S::Future: 'static,
    B: 'static
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError=();
    type Transform = DbTransactionMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(DbTransactionMiddleware{
            service: Rc::new(RefCell::new(service))
        })
    }
}


/// The resource in question's Timestamp
pub struct ResourceTimestamp(SyncTimestamp);

/*
#[derive(Debug)]
pub struct PreConditionCheck;

impl PreConditionCheck {
    pub fn new() -> Self {
        PreConditionCheck::default()
    }
}

impl Default for PreConditionCheck {
    fn default() -> Self {
        Self
    }
}

impl<S, B> Transform<S> for PreConditionCheck
where
//B: MessageBody,
S: Service<Request =ServiceRequest, Response=ServiceResponse<B>, Error= Error>,
S::Future: 'static,
//    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError=();
    type Transform = PreConditionCheckMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(PreConditionCheckMiddleware{
            service
        })
    }
}

pub struct PreConditionCheckMiddleware<S> {
    service: S,
}

impl<S, B> Service for PreConditionCheckMiddleware<S>
where
//B: MessageBody,
S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error= Error>,
S::Future: 'static,
//    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    //type Future = Either<FutureResult<Self::Response, Self::Error>, S::Future>;

    // call super poll_ready()
    fn poll_ready(&mut self) -> Poll<(), Self::Error>{
        self.service.poll_ready()
    }

    fn call(&mut self, sreq: ServiceRequest) -> Self::Future {
        let (req, mut payload) = sreq.into_parts();
        // Pre check
        let precondition = match PreConditionHeaderOpt::from_request(&req, &mut payload) {
            Ok(precond) =>
                match precond.opt {
                    Some(p) => p,
                    // XXX: not responses, just continue on
                    None => return Box::new(future::ok(ServiceResponse::new(req, HttpResponse::Ok().finish().into_body())))
                },
            Err(e) => {
                // XXX: same
                return Box::new(future::ok(ServiceResponse::new(
                    req,
                    HttpResponse::InternalServerError().body(format!("Err: {:?}", e)).into_body()),
                ))
            }
        };

        let user_id = HawkIdentifier::from_request(&req, &mut payload).unwrap();
        let db = <Box<dyn Db>>::from_request(&req, &mut payload);
        let collection = CollectionParam::from_request(&req, &mut payload).ok().map(|v| v.collection);
        let bso = BsoParam::from_request(&req, &mut payload).ok();

        db.unwrap()
            .extract_resource(user_id, collection, bso)
            .map_err(Into::into)
            .and_then(move |resource_ts: SyncTimestamp| {
                req.extensions_mut().insert(ResourceTimestamp(resource_ts));
                let status = match precondition {
                    PreConditionHeader::IfModifiedSince(header_ts) if resource_ts <= header_ts => {
                        StatusCode::NOT_MODIFIED
                    }
                    PreConditionHeader::IfUnmodifiedSince(header_ts) if resource_ts > header_ts => {
                        StatusCode::PRECONDITION_FAILED
                    }
                    _ => StatusCode::OK,
                };
                if status != StatusCode::OK {
                    return Box::new(future::ok(ServiceResponse::new(
                        req,
                        HttpResponse::build(status)
                        .header("X-Last-Modified", resource_ts.as_header())
                        .body("").into_body()
                    )));
                };
                Box::new(future::ok(ServiceResponse::new(req, HttpResponse::Ok().finish().into_body())))
            }).and_then(

        // post-check

        Box::new(self.service.call(sreq).and_then(|mut sresp| {
            let resp = sresp.response();
            if sresp.headers().contains_key("X-Last-Modified") {
                return Either::A(future::ok(ServiceResponse::new(req,
                HttpResponse::build(StatusCode::OK).finish().into_body())))
            }

                // See if we already extracted one and use that if possible
            if let Some(resource_ts) = req.extensions().get::<ResourceTimestamp>() {
                let ts = resource_ts.0;
                if let Ok(ts_header) = header::HeaderValue::from_str(&ts.as_header()) {
                    resp.headers_mut().insert(header::HeaderName::from_static("X-Last-Modified"), ts_header);
                }
                return Either::A(future::ok(ServiceResponse::new(
                    req,
                    HttpResponse::build(StatusCode::OK).finish().into_body(),
                )));
            }

            // Do the work needed to generate a timestamp otherwise
            let user_id = HawkIdentifier::from_request(&req, &mut payload).unwrap();
            let db = <Box<dyn Db>>::from_request(&req, &mut payload).unwrap();
            let collection = CollectionParam::from_request(&req, &mut payload)
                .ok()
                .map(|v| v.collection);
            let bso = BsoParam::from_request(&req, &mut payload).ok(); //.map(|v| v.bso);
            let fut = db
                .extract_resource(user_id, collection, bso)
                .map_err(|e| {
                    let e: Error = e.into();
                    e
                })
                .and_then(move |resource_ts: SyncTimestamp| {
                    if let Ok(ts_header) = header::HeaderValue::from_str(&resource_ts.as_header()) {
                        resp.headers_mut().insert(header::HeaderName::from_static("X-Last-Modified"), ts_header);
                    }
                    return Box::new(future::ok(ServiceResponse::new(req, *resp.clone())));
                });
                //.map_err(Into::into);
            return Either::B(fut);
        }))
    }
}
*/

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
