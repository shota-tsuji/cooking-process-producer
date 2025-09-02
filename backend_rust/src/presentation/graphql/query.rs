use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, ID};
use sea_orm::DatabaseConnection;
use std::collections::{HashMap, HashSet};

use crate::presentation::graphql::mutation::Mutation;
use crate::presentation::graphql::object::{
    HelloResponse, Process, Resource, ResourceInfo, StepResult,
};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use crate::infrastructure::mysql::entity as db_entity;
use crate::presentation::graphql::query::hello_world::StepOutput;
use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloRequest, ProcessRequest};
use sea_orm::*;

use super::object::{Recipe, RecipeDetail, Step};

pub type QuerySchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query {
    db: DatabaseConnection,
}

impl Query {
    pub fn new(db: DatabaseConnection) -> Self {
        Query { db }
    }
}

#[Object]
impl Query {
    async fn recipe_detail(&self, _ctx: &Context<'_>, id: ID) -> Result<RecipeDetail, String> {
        let recipe: Recipe = db_entity::recipes::Entity::find_by_id(id.as_str().to_string())
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .map(|model| Recipe {
                id: model.id,
                title: model.title,
                description: model.description.unwrap_or_default(),
            })
            .ok_or_else(|| "Recipe not found".to_string())?;
        let steps: Vec<Step> = db_entity::steps::Entity::find()
            .filter(db_entity::steps::Column::RecipeId.eq(id.as_str().to_string()))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|model| Step {
                id: model.id,
                description: model.description,
                resource_id: model.resource_id,
                order_number: model.order_number,
                duration: model.duration,
            })
            .collect();

        let recipe_detail = RecipeDetail {
            id: recipe.id,
            title: recipe.title,
            description: recipe.description,
            steps,
        };

        println!("{:?}", &recipe_detail);

        Ok(recipe_detail)
    }

    async fn recipes(&self, _ctx: &Context<'_>) -> Result<Vec<Recipe>, String> {
        let rs: Vec<db_entity::recipes::Model> = db_entity::recipes::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;
        let recipes = rs
            .into_iter()
            .map(|recipe| Recipe {
                id: recipe.id,
                title: recipe.title,
                description: recipe.description.unwrap_or_default(),
            })
            .collect();
        println!("rs: {:?}", recipes);

        Ok(recipes)
    }

    async fn process(&self, _ctx: &Context<'_>, id: ID) -> Result<Process, String> {
        let mut client = GreeterClient::connect("http://main:50051").await.unwrap();

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
                    resource_id: step.resource_id.clone(),
                    duration: step.duration,
                    order_number: step.order_number,
                })
                .collect();
            grpc_recipes.push(hello_world::Recipe {
                id: recipe.id.clone(),
                steps: grpc_steps,
            });
        }

        let mut resource_models: Vec<db_entity::resources::Model> =
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
            recipes: grpc_recipes.into(),
            resources: grpc_resources.into(),
        });

        let mut response = client.process(request).await.unwrap();
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
        println!("{:?}", resource_infos);

        let process = Process { resource_infos };

        println!("step4");
        //Ok(recipeDetails)
        Ok(process)
    }

    async fn resource(&self, _ctx: &Context<'_>, id: ID) -> Result<Resource, String> {
        let model: db_entity::resources::Model =
            db_entity::resources::Entity::find_by_id(id.as_str().parse::<u64>().unwrap())
                .one(&self.db)
                .await
                .map_err(|e| e.to_string())?
                .ok_or_else(|| "Resource not found".to_string())?;
        let resource = Resource {
            id: model.id,
            name: model.name,
            amount: model.amount,
        };

        println!("{:?}", &resource);

        Ok(resource)
    }

    async fn resources(&self, _ctx: &Context<'_>) -> Result<Vec<Resource>, String> {
        let models: Vec<db_entity::resources::Model> = db_entity::resources::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;
        let resources = models
            .into_iter()
            .map(|resource| Resource {
                id: resource.id,
                name: resource.name,
                amount: resource.amount,
            })
            .collect::<Vec<Resource>>();

        Ok(resources)
    }
}
