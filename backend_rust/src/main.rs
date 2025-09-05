use async_graphql::{EmptySubscription, Schema};

use axum::routing::post;
use axum::{extract::Extension, Router};
use cpp_backend::presentation::{
    controller::graphql_controller::graphql_handler,
    graphql::{mutation::Mutation, query::Query},
};
use http::{
    header::{ACCEPT, CONTENT_TYPE},
    Method,
};
use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;
use std::env;
use std::fs::File;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}
#[derive(Debug, Deserialize)]
pub struct Config {
    pub socket_addr: String,
    pub database_url: String,
    pub origins: Vec<String>,
}
#[tokio::main]
async fn main() {
    let config_file_path = env::var("CONFIG_FILE").unwrap();
    let config_file = File::open(config_file_path).unwrap();
    let config: Config = serde_yaml::from_reader(config_file).unwrap();

    let ops = ConnectOptions::new(config.database_url);
    let db = Database::connect(ops.clone()).await.unwrap();
    let db2 = Database::connect(ops.clone()).await.unwrap();

    let query = Query::new(db);
    let mutation = Mutation::new(db2);
    let schema = Schema::build(query, mutation, EmptySubscription).finish();

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
        .layer(Extension(schema));

    axum::Server::bind(&config.socket_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
