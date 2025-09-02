use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use cpp_backend::presentation::graphql::{mutation::Mutation, query::Query};
use sea_orm::{ConnectOptions, Database};
use sqlx::MySqlPool;
use std::env;

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let database_url = env::var("DATABASE_URL").unwrap();
    let mut ops = ConnectOptions::new(database_url.clone());
    let db = Database::connect(ops).await.unwrap();
    let query = Query::new(db);
    let mutation = Mutation::new(pool.clone());
    let schema = Schema::build(query, mutation, EmptySubscription).finish();
    print!("{}", schema.sdl());
}
