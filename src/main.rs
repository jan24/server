mod analysis;
mod template;
mod view;

use axum::extract::Request;
use axum::{routing, Router, ServiceExt};
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::services::fs::ServeDir;
use tower_layer::Layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use view::{homepage, line_page, keyname, query_sn, portconfig, query_cell, pf_data, day_yield, fail_detail,
           pre_day, pre_shift, json_today};
use crate::analysis::config::CONFIG;

#[tokio::main]
async fn main() {
    analysis::config::init_config();
    analysis::lang_tran::init_lang_map();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_templates=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    println!("tracing init complete");
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", routing::get(homepage))
        .route("/json/today", routing::get(json_today))
        .route("/:lang/:line", routing::get(line_page))
        .route("/:lang/:line/portconfig", routing::get(portconfig))
        .route("/:lang/:line/keyname", routing::get(keyname))
        .route("/:lang/:line/query_sn", routing::get(query_sn))
        .route("/:lang/:line/pf_data", routing::get(pf_data))
        .route("/:lang/:line/day_yield", routing::get(day_yield))
        .route("/:lang/:line/fail_detail", routing::get(fail_detail))
        .route("/:lang/:line/query_cell", routing::get(query_cell))
        .route("/:lang/:line/:item/preday", routing::get(pre_day))
        .route("/:lang/:line/:item/preshift", routing::get(pre_shift));

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    // let app = app.fallback(handler_404);
    let port = CONFIG.get().unwrap().port;
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}
