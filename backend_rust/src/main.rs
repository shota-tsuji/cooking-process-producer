use async_graphql::{EmptySubscription, Schema};

use axum::routing::post;
use axum::{Router, extract::Extension};
use cpp_backend::infrastructure::db::db_recipe_repository::DbRecipeRepository;
use cpp_backend::infrastructure::db::db_resource_repository::DbResourceRepository;
use cpp_backend::presentation::{
    controller::graphql_controller::graphql_handler,
    graphql::{mutation::Mutation, query::Query},
};
use http::{
    Method,
    header::{ACCEPT, CONTENT_TYPE},
};
use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::{DefaultOnRequest, TraceLayer};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

pub mod hello_world {
    tonic::include_proto!("proto.cooking.v1");
}
#[derive(Debug, Deserialize)]
pub struct Config {
    pub socket_addr: String,
    pub database_url: String,
    pub origins: Vec<String>,
}
#[tokio::main]
async fn main() {
    // Setup logging
    //LogTracer::init().expect("Failed to set logger");
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=debug,async_graphql=debug".into());
    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_span_events(FmtSpan::CLOSE)
                .with_current_span(true)
                .with_target(false)
                .with_thread_ids(true)
                .with_thread_names(true),
        )
        .init();

    let config_file_path = env::var("CONFIG_FILE").unwrap();
    let config_file = File::open(config_file_path).unwrap();
    let config: Config = serde_yaml::from_reader(config_file).unwrap();

    let ops = ConnectOptions::new(config.database_url);
    let db2 = Database::connect(ops.clone()).await.unwrap();
    let db3 = Database::connect(ops.clone()).await.unwrap();
    let db4 = Database::connect(ops.clone()).await.unwrap();
    let resource_repository = Arc::new(DbResourceRepository { db_connection: db3 });
    let recipe_repository = Arc::new(DbRecipeRepository { db_connection: db4 });

    let query = Query {};
    let mutation = Mutation::new(db2);
    let schema = Schema::build(query, mutation, EmptySubscription)
        .data(resource_repository.clone())
        .data(recipe_repository.clone())
        .finish();

    let cors = CorsLayer::new()
        .allow_origin(
            config
                .origins
                .iter()
                .map(|o| o.parse().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods([Method::POST])
        .allow_headers(vec![ACCEPT, CONTENT_TYPE]);
    let cors_layer = ServiceBuilder::new().layer(cors);

    let app = Router::new()
        .route("/", post(graphql_handler))
        .layer(cors_layer)
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        // Add tracing layer for structured logging
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &http::Request<_>| {
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");

                    tracing::info_span!(
                        "http_request",
                        method = ?request.method(),
                        uri = ?request.uri(),
                        request_id = %request_id
                    )
                })
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO)),
        )
        .layer(Extension(schema));

    axum::Server::bind(&config.socket_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
