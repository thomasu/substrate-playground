#![feature(async_closure, proc_macro_hygiene, decl_macro)]
#![deny(intra_doc_link_resolution_failure)]

mod api;
mod kubernetes;
mod manager;
mod metrics;
mod template;

use crate::manager::Manager;
use rocket::{config::Environment, http::Method, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_prometheus::PrometheusMetrics;
use std::{env, error::Error};

pub struct Context {
    manager: Manager,
}

/// manager -> kubernetes, metrics
/// manager is injected into api
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let manager = Manager::new().await?;
    manager.clone().spawn_background_thread();
    let prometheus = PrometheusMetrics::with_registry(manager.clone().metrics.create_registry()?);
    let error = rocket::ignite()
        .attach(prometheus.clone())
        .attach(cors)
        .mount(
            "/api",
            routes![
                api::deploy,
                api::get,
                api::get_instance,
                api::get_templates,
                api::list,
                api::undeploy
            ],
        )
        .mount("/metrics", prometheus)
        .manage(Context { manager })
        .launch();

    // Launch blocks unless an error is returned
    Err(error.into())
}
