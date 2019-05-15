//! Main application server

use std::sync::Arc;

use actix::{System, SystemRunner};
use actix_web::{middleware::cors::Cors, web, App, HttpRequest, HttpResponse, HttpServer};
//use num_cpus;

use crate::web::handlers;
use crate::web::middleware;
use db::{mysql::MysqlDbPool, DbError, DbPool};
use settings::{Secrets, ServerLimits, Settings};

fn init_routes(s_config: &mut web::ServiceConfig) {
    s_config
        .service(
            web::resource("/1.5/{uid}/info/collections")
                .route(web::get().to_async(handlers::get_collections)),
        )
        .service(
            web::resource("/1.5/{uid}/info/collection_counts")
                .route(web::get().to_async(handlers::get_collection_counts)),
        )
        .service(
            web::resource("/1.5/{uid}/info/collection_usage")
                .route(web::get().to_async(handlers::get_collection_usage)),
        )
        /* TODO:  Needs FromRequest for get_configuration
        .service(
            web::resource("/1.5/{uid}/info/configuration")
                .route(web::get().to(handlers::get_configuration)),
        )
        */
        .service(web::resource("/1.5/{uid}/info/quota").route(web::get().to_async(handlers::get_quota)))
        .service(web::resource("/1.5/{uid}").route(web::delete().to_async(handlers::delete_all)))
        .service(web::resource("/1.5/{uid}/storage").route(web::delete().to_async(handlers::delete_all)))
        .service(
            web::resource("/1.5/{uid}/storage/{collection}")
                .route(web::delete().to_async(handlers::delete_collection))
                .route(web::get().to_async(handlers::get_collection))
                .route(web::post().to_async(handlers::post_collection)),
        )
        .service(
            web::resource("/1.5/{uid}/storage/{collection}/{bso}")
                .route(web::delete().to_async(handlers::delete_bso))
                .route(web::get().to_async(handlers::get_bso))
                .route(web::put().to_async(handlers::put_bso)),
        );
}

// The tests depend on the init_routes! macro, so this mod must come after it
#[cfg(test)]
mod test;

use actix_web::dev::Body;

/// This is the global HTTP state object that will be made available to all
/// HTTP API calls.
pub struct ServerState {
    pub db_pool: Box<dyn DbPool>,

    /// Server-enforced limits for request payloads.
    pub limits: Arc<ServerLimits>,

    /// Secrets used during Hawk authentication.
    pub secrets: Arc<Secrets>,

    pub port: u16,
}

pub fn build_app(state: ServerState) -> App<impl actix_service::NewService, Body> {
    App::new()
        .data(state)
        .wrap(middleware::WeaveTimestamp::default())
        .wrap(middleware::DbTransaction::default())
        .wrap(middleware::PreConditionCheck::default())
        .wrap(Cors::default())
        .configure(init_routes)
}

pub fn build_dockerflow(state: ServerState) -> App<impl actix_service::NewService, Body> {
    App::new()
        .data(state)
        // Handle the resource that don't need to go through middleware
        .service(web::resource("/__heartbeat__").route(web::get().to(|_:HttpRequest| {
            // if addidtional information is desired, point to an appropriate handler.
            let body = json!({"status": "ok", "version": env!("CARGO_PKG_VERSION")});
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body.to_string())
        })))
        .service(web::resource("/__lbheartbeat__").route(web::get().to(|_:HttpRequest| {
            // used by the load balancers, just return OK.
            HttpResponse::Ok()
                .content_type("application/json")
                .body("{}")
        })))
        .service(web::resource("/__version__").route(web::get().to(|_:HttpRequest| {
            // return the contents of the version.json file created by circleci and stored in the docker root
            HttpResponse::Ok()
                .content_type("application/json")
                .body(include_str!("../../version.json"))
        })))
}

pub struct Server {}

impl Server {
    pub fn with_settings(settings: Settings) -> Result<SystemRunner, DbError> {
        let sys = System::new("syncserver");
        let db_pool = Box::new(MysqlDbPool::new(&settings)?);
        let limits = Arc::new(settings.limits);
        let secrets = Arc::new(settings.master_secret);
        let port = settings.port;

        HttpServer::new(move || {
            // Setup the server state
            let state = ServerState {
                db_pool: db_pool.clone(),
                limits: Arc::clone(&limits),
                secrets: Arc::clone(&secrets),
                port,
            };

            build_app(state).into()
        })
        .bind(format!("127.0.0.1:{}", settings.port))
        .unwrap()
        .start();
        Ok(sys)
    }
}
