use crate::adapters::repository::mysql::entity as db_entity;
use crate::adapters::repository::mysql::resource_mapper::ResourceMapper;
use crate::application::mapper::db_mapper::DbMapper;
use crate::application::port::repository::ResourceRepository;
use crate::domain::Resource;
use crate::domain::entity::resource::ResourceId;
use crate::domain::error::AsyncDynError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::sync::Arc;

pub struct MysqlResourceRepository {
    pub db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
impl ResourceRepository for MysqlResourceRepository {
    async fn get_resource_by_id(&self, id: i32) -> Result<Resource, Box<AsyncDynError>> {
        let model = db_entity::resources::Entity::find_by_id(id as u64)
            .one(&*self.db_connection)
            .await;

        match model {
            Ok(Some(model)) => {
                let resource = Resource {
                    id: ResourceId(model.id as i32),
                    name: model.name,
                    amount: model.amount,
                };
                Ok(resource)
            }
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                e,
            ))),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Resource not found",
            ))),
        }
    }

    async fn get_resources_by_ids(
        &self,
        resource_ids: Vec<ResourceId>,
    ) -> Result<Vec<Resource>, Box<AsyncDynError>> {
        use db_entity::resources;
        let ids: Vec<u64> = resource_ids.iter().map(|rid| rid.0 as u64).collect();
        let models = resources::Entity::find()
            .filter(resources::Column::Id.is_in(ids))
            .all(&*self.db_connection)
            .await
            .map_err(|e| Box::new(std::io::Error::other(e)))?;

        let resources: Vec<Resource> = models.into_iter().map(ResourceMapper::to_entity).collect();
        Ok(resources)
    }

    async fn get_all_resources(&self) -> Result<Vec<Resource>, Box<AsyncDynError>> {
        let models = db_entity::resources::Entity::find()
            .all(&*self.db_connection)
            .await;

        match models {
            Ok(models) => {
                let models = models
                    .into_iter()
                    .map(|model| Resource {
                        id: ResourceId(model.id as i32),
                        name: model.name,
                        amount: model.amount,
                    })
                    .collect::<Vec<Resource>>();
                Ok(models)
            }
            Err(e) => Err(Box::new(std::io::Error::other(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sea_orm::Database;
    use std::sync::Arc;
    //use testcontainers::{clients, RunnableImage};
    use testcontainers_modules::mysql::Mysql;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;
    use tokio;

    #[tokio::test]
    async fn test_get_resource_by_id() {
        // Start MySQL container
        //let docker = clients::Cli::default();
        let node = AsyncRunner::start(Mysql::default()).await.unwrap();
        //let node = docker.run(node);

        println!(
            "MySQL is running on port: {}",
            node.get_host_port_ipv4(3306).await.unwrap()
        );

        let port = node.get_host_port_ipv4(3306).await.unwrap();
        let host = node.get_host().await.unwrap();
        let url = format!("mysql://root@{}:{}/test", host, port,);

        println!("URL: {}", url);
        // Wait for MySQL to be ready
        let db = loop {
            match Database::connect(&url).await {
                Ok(conn) => break conn,
                Err(e) => {
                    println!("Error connecting to MySQL: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await
                }
            }
            println!("Waiting for MySQL to be ready...");
        };
        println!("Connected to MySQL");

        migration::seed::seed_resource_repository_medium_test(&db).await;
        /*
        let migration_img = RunnableImage::from("my-seaorm-migration:latest")
            .with_env_var("DATABASE_URL", &db_url)
            .with_cmd(vec!["up"]); // run `migration up`
        let migration_img = Image::name("my-seaorm-migration")
        let _migrator = docker.run(migration_img);
         */
        // Create table
        /*
        let _schema = Schema::new(DbBackend::MySql);
        //let stmt = schema.create_table_from_entity(resources::Entity).to_string();
        let q = "CREATE TABLE IF NOT EXISTS resources (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(140) NOT NULL,
    amount INT NOT NULL,
    PRIMARY KEY (id)
);
";
        db.execute(Statement::from_string(DbBackend::MySql, q))
            .await
            .unwrap();

        // Insert test data
        let resource = resources::ActiveModel {
            id: Set(1),
            name: Set("Sugar".to_string()),
            amount: Set(2),
        };
        resource.insert(&db).await.unwrap();
         */

        let repo = MysqlResourceRepository {
            db_connection: Arc::new(db),
        };

        let result = repo.get_resource_by_id(1).await.unwrap();
        assert_eq!(result.id, ResourceId(1));
        assert_eq!(result.name, "Sugar");
        assert_eq!(result.amount, 2);
    }
}
