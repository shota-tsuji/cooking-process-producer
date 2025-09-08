use async_graphql::{EmptySubscription, Schema};
use cpp_backend::presentation::graphql::{mutation::Mutation, query::Query};
use sea_orm::{ConnectOptions, Database};
use std::env;

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let ops = ConnectOptions::new(database_url.clone());
    let db2 = Database::connect(ops.clone()).await.unwrap();
    let query = Query {};
    let mutation = Mutation::new(db2);
    let schema = Schema::build(query, mutation, EmptySubscription).finish();
    print!("{}", schema.sdl());
}
