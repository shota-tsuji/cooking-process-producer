use async_graphql::{EmptySubscription, Schema};

use axum::{extract::Extension, routing::get, Router};
use cpp_backend::presentation::{
    controller::graphql_controller::{graphiql, graphql_handler},
    graphql::{mutation::Mutation, query::Query},
};
use http::{
    header::{ACCEPT, CONTENT_TYPE},
    Method,
};
use sea_orm::{ConnectOptions, Database};
use std::env;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let ops = ConnectOptions::new(database_url.clone());
    let db = Database::connect(ops.clone()).await.unwrap();
    let db2 = Database::connect(ops.clone()).await.unwrap();

    let query = Query::new(db);
    let mutation = Mutation::new(db2);
    let schema = Schema::build(query, mutation, EmptySubscription).finish();

    let origins = ["http://localhost:8000".parse().unwrap()];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::POST])
        .allow_headers(vec![ACCEPT, CONTENT_TYPE]);
    let cors_layer = ServiceBuilder::new().layer(cors);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(cors_layer)
        .layer(Extension(schema));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
