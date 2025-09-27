use async_graphql::{EmptySubscription, Schema};
use axum::routing::post;
use axum::{Router, extract::Extension};
use cpp_backend::adapters::controller::graphql::graphql_controller::graphql_handler;
use cpp_backend::adapters::controller::graphql::mutation::Mutation;
use cpp_backend::adapters::controller::graphql::query::Query;
use cpp_backend::adapters::external::grpc::cooking::process_service_client;
use cpp_backend::adapters::external::grpc::process_service_client::GrpcProcessServiceClient;
use cpp_backend::adapters::repository::MysqlProcessRepository;
use cpp_backend::adapters::repository::MysqlRecipeRepository;
use cpp_backend::adapters::repository::MysqlResourceRepository;
use http::{
    Method,
    header::{ACCEPT, CONTENT_TYPE},
};
use sea_orm::{ConnectOptions, Database};
use secrecy::{ExposeSecret, SecretBox, SecretString};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::{DefaultOnRequest, TraceLayer};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

pub mod proto {
    tonic::include_proto!("proto.cooking.v1");
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self, password: SecretString) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "mysql://{}:{}@{}:{}/cooking",
            self.username,
            password.expose_secret(),
            self.host,
            self.port
        )))
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub socket_addr: String,
    pub database: DatabaseConfig,
    pub origins: Vec<String>,
    pub process_grpc_server_url: String,
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

    let password_path = env::var("DATABASE_PASSWORD_PATH").unwrap();
    let password_file = File::open(password_path).unwrap();
    let password = std::io::read_to_string(password_file).unwrap();
    let password = SecretString::new(Box::from(password));
    let ops = ConnectOptions::new(config.database.connection_string(password).expose_secret());
    //let db = Arc::new(Database::connect(ops.clone()).await.unwrap());
    let db = match Database::connect(ops.clone()).await {
        Ok(conn) => Arc::new(conn),
        Err(e) => {
            tracing::error!("Failed to connect to database: {:?}", e);
            panic!("Database connection error: {:?}", e);
        }
    };
    let db2 = match Database::connect(ops.clone()).await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to connect to database (db2): {:?}", e);
            panic!("Database connection error (db2): {:?}", e);
        }
    };

    //let db2 = Database::connect(ops.clone()).await.unwrap();
    let resource_repository = Arc::new(MysqlResourceRepository {
        db_connection: db.clone(),
    });
    let recipe_repository = Arc::new(MysqlRecipeRepository {
        db_connection: db.clone(),
    });
    let process_registration_repository = Arc::new(MysqlProcessRepository {
        db_connection: db.clone(),
    });
    let process_client = Arc::new(Mutex::new(
        process_service_client::ProcessServiceClient::connect(config.process_grpc_server_url)
            .await
            .unwrap(),
    ));
    let process_service = Arc::new(GrpcProcessServiceClient {
        client: process_client.clone(),
    });

    let query = Query {};
    let mutation = Mutation::new(db2);
    let schema = Schema::build(query, mutation, EmptySubscription)
        .data(resource_repository.clone())
        .data(recipe_repository.clone())
        .data(process_registration_repository.clone())
        .data(process_service.clone())
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
