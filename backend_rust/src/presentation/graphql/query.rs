use async_graphql::{Context, EmptySubscription, ID, Object, Schema};
use std::sync::Arc;

use crate::adapters::db::db_recipe_repository::DbRecipeRepository;
use crate::adapters::db::db_resource_repository::DbResourceRepository;
use crate::adapters::recipe_mapper::{RecipeDetailMapper, RecipeMapper};
use crate::application::mapper::api_mapper::ApiMapper;
use crate::application::usecase::get_all_resources_usecase::GetAllResourcesUsecase;
use crate::application::usecase::get_one_recipe_by_id_usecase::GetOneRecipeByIdUseCase;
use crate::application::usecase::get_one_resource_by_id_usecase::GetOneResourceByIdUseCase;
use crate::application::usecase::interface::AbstractUseCase;
use crate::presentation::graphql::mutation::Mutation;
use crate::presentation::graphql::object::{Process, Resource, ResourceInfo, StepResult};

//pub mod hello_world {
//    tonic::include_proto!("proto", "cooking", "v1");
//}

use super::object::{Recipe, RecipeDetail};

pub type QuerySchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query {}

#[Object]
impl Query {
    async fn recipe_detail(&self, ctx: &Context<'_>, id: ID) -> Result<RecipeDetail, String> {
        let repository = ctx
            .data::<Arc<DbRecipeRepository>>()
            .map_err(|_| "Repository not found".to_string())?;

        let id = id.to_string();
        let usecase = GetOneRecipeByIdUseCase::new(&id, repository.as_ref());
        let result = usecase.execute().await;
        result
            .map_err(|e| e.message)
            .map(RecipeDetailMapper::to_api)
    }

    async fn recipes(&self, _ctx: &Context<'_>) -> Result<Vec<Recipe>, String> {
        let repository = _ctx
            .data::<Arc<DbRecipeRepository>>()
            .map_err(|_| "Repository not found".to_string())?;
        let use_case =
            crate::application::usecase::get_all_recipes_usecase::GetAllRecipesUseCase::new(
                repository.as_ref(),
            );

        let recipes_result = use_case.execute().await;

        let Ok(recipes) = recipes_result else {
            return Err("Failed to get recipes".to_string());
        };
        Ok(recipes.into_iter().map(RecipeMapper::to_api).collect())
    }

    async fn process(&self, _ctx: &Context<'_>, _id: ID) -> Result<Process, String> {
        let steps_0 = vec![StepResult {
            id: "abc".to_string(),
            recipe_id: String::from("00000000-0000-0000-0000-000000000000"),
            resource_id: 1u64,
            start_time: 10,
            duration: 5,
            order_number: 0,
            timeline_index: 1,
            description: String::from("野菜を切る"),
            recipe_name: String::from("カレー"),
        }];
        let resource_info0 = ResourceInfo {
            id: 1u64,
            name: String::from("人手"),
            steps: steps_0,
        };
        let resource_infos = vec![resource_info0];
        let process = Process { resource_infos };
        Ok(process)

        /*
        let mut _client = GreeterClient::connect("http://main:50051").await.unwrap();

        println!("step0");
        let recipe_models: Vec<db_entity::recipes::Model> = db_entity::recipes::Entity::find()
            .join(
                JoinType::LeftJoin,
                db_entity::process_regsitrations::Entity::belongs_to(db_entity::recipes::Entity)
                    .from(db_entity::process_regsitrations::Column::RecipeId)
                    .to(db_entity::recipes::Column::Id)
                    .into(),
            )
            .filter(db_entity::process_regsitrations::Column::ProcessId.eq(id.as_str().to_string()))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;
        let recipes: Vec<Recipe> = recipe_models
            .iter()
            .map(|recipe| Recipe {
                id: recipe.id.clone(),
                title: recipe.title.clone(),
                description: recipe.description.clone().unwrap_or_default(),
            })
            .collect();

        println!("step1");
        let mut resource_set = HashSet::new();
        let mut grpc_recipes: Vec<hello_world::Recipe> = Vec::new();
        let mut step_infos: HashMap<String, String> = HashMap::new();
        let mut recipe_infos: HashMap<String, String> = HashMap::new();
        for recipe in &recipes {
            let step_models: Vec<db_entity::steps::Model> = db_entity::steps::Entity::find()
                .filter(db_entity::steps::Column::RecipeId.eq(recipe.id.clone()))
                .all(&self.db)
                .await
                .map_err(|e| e.to_string())?;
            let steps: Vec<Step> = step_models
                .iter()
                .map(|step| Step {
                    id: step.id.clone(),
                    description: step.description.clone(),
                    resource_id: step.resource_id,
                    order_number: step.order_number,
                    duration: step.duration,
                })
                .collect();

            println!("step2");
            recipe_infos.insert(recipe.id.clone(), recipe.title.clone());

            // Get unique resource ids
            for step in &steps {
                resource_set.insert(step.resource_id);
                step_infos.insert(step.id.clone(), step.description.clone());
            }

            let grpc_steps = steps
                .iter()
                .map(|step| hello_world::Step {
                    id: step.id.clone(),
                    recipe_id: recipe.id.clone(),
                    resource_id: step.resource_id,
                    duration: step.duration,
                    order_number: step.order_number,
                })
                .collect();
            grpc_recipes.push(hello_world::Recipe {
                id: recipe.id.clone(),
                steps: grpc_steps,
            });
        }

        let resource_models: Vec<db_entity::resources::Model> =
            db_entity::resources::Entity::find()
                .all(&self.db)
                .await
                .map_err(|e| e.to_string())?;
        let mut resources: Vec<Resource> = resource_models
            .iter()
            .map(|resource| Resource {
                id: resource.id,
                name: resource.name.clone(),
                amount: resource.amount,
            })
            .collect();
        resources.sort_by_key(|r| r.id);
        println!("{:?}", resources);

        println!("step3");
        // Filter only used resources
        let grpc_resources: Vec<hello_world::Resource> = resources
            .iter()
            .filter(|&resource| resource_set.contains(&resource.id))
            .map(|resource| hello_world::Resource {
                id: resource.id,
                amount: resource.amount,
            })
            .collect();

        let request = tonic::Request::new(ProcessRequest {
            recipes: grpc_recipes,
            resources: grpc_resources,
        });

        let mut response = _client.process(request).await.unwrap();
        println!("{:?}", response.get_ref().steps);
        println!("{:?}", response.get_ref().resource_infos);

        let step_results: Vec<StepResult> = response
            .get_ref()
            .steps
            .iter()
            .map(|step: &StepOutput| {
                let description = step_infos.get(step.step_id.as_str()).unwrap();
                let recipe_name = recipe_infos.get(step.recipe_id.as_str()).unwrap();
                StepResult {
                    id: step.step_id.clone(),
                    recipe_id: step.recipe_id.clone(),
                    resource_id: step.resource_id,
                    start_time: step.start_time,
                    duration: step.duration,
                    order_number: 0,
                    timeline_index: step.time_line_index,
                    description: description.to_string(),
                    recipe_name: recipe_name.to_string(),
                }
            })
            .collect();

        response.get_mut().resource_infos.sort_by_key(|r| r.id);
        let mut resource_infos: Vec<ResourceInfo> = Vec::new();
        for (i, resource) in response.get_ref().resource_infos.iter().enumerate() {
            for j in 0..resource.used_resources_count {
                let mut steps: Vec<StepResult> = Vec::new();
                for step in &step_results {
                    if step.resource_id == resource.id as u64 && j == step.timeline_index {
                        steps.push(step.clone());
                    }
                }

                resource_infos.push(ResourceInfo {
                    id: resource.id as u64,
                    name: resources[i].name.clone(),
                    steps,
                });
            }
        }

        let process = Process { resource_infos };
        Ok(process)
         */
    }

    async fn resource(&self, ctx: &Context<'_>, id: ID) -> Result<Resource, String> {
        // Extract repository instance from context
        let repository = ctx
            .data::<Arc<DbResourceRepository>>()
            .map_err(|_| "Repository not found".to_string())?;

        // Parse id to i32 (adjust if your domain uses u64)
        let resource_id = id
            .as_str()
            .parse::<i32>()
            .map_err(|_| "Invalid id".to_string())?;

        // Create and execute usecase
        let usecase = GetOneResourceByIdUseCase::new(&resource_id, repository.as_ref());
        let result = usecase.execute().await;

        // Map usecase result to GraphQL Resource
        result
            .map(|r| Resource {
                id: r.id as u64,
                name: r.name,
                amount: r.amount,
            })
            .map_err(|e| e.message)
    }

    async fn resources(&self, _ctx: &Context<'_>) -> Result<Vec<Resource>, String> {
        let repository = _ctx
            .data::<Arc<DbResourceRepository>>()
            .map_err(|_| "Repository not found".to_string())?;
        let usecase = GetAllResourcesUsecase::new(repository.as_ref());
        let result = usecase.execute().await;
        let resources = result
            .map_err(|e| e.message)?
            .into_iter()
            .map(|r| Resource {
                id: r.id as u64,
                name: r.name,
                amount: r.amount,
            })
            .collect();

        Ok(resources)
    }
}

#[cfg(test)]
mod tests {

    use crate::infrastructure::mysql::entity as db_entity;
    use sea_orm::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[async_std::test]
    async fn test_example() -> Result<(), DbErr> {
        let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![db_entity::recipes::Model {
                id: "1".to_string(),
                title: "Test Recipe".to_string(),
                description: Some("A test recipe".to_string()),
            }]])
            .into_connection();
        //let query = Query::new(db);
        //let schema = Schema::build(query, EmptyMutation, EmptySubscription).finish();
        //let response = schema.execute(
        //    r#"
        //        query {
        //          recipes {
        //            id
        //            title
        //            description
        //          }
        //        }
        //    "#,
        //).await
        //    .into_result()
        //    .unwrap();
        let expected: Vec<db_entity::recipes::Model> = vec![db_entity::recipes::Model {
            id: "1".to_string(),
            title: "Test Recipe".to_string(),
            description: Some("A test recipe".to_string()),
        }];
        assert_eq!(db_entity::recipes::Entity::find().all(&db).await?, expected);
        Ok(())
    }
}
