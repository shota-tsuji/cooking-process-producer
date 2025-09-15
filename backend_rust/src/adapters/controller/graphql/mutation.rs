use crate::adapters::controller::graphql::object::{
    CreateProcessInput, CreateResourceInput, CreateStepInput, ProcessId, Resource,
    UpdateResourceInput,
};
use crate::adapters::controller::graphql::object::{CreateRecipeDetailInput, RecipeDetail, Step};
use crate::adapters::repository::MysqlProcessRepository;
use crate::adapters::repository::MysqlRecipeRepository;
use crate::adapters::repository::MysqlResourceRepository;
use crate::application::usecase::calculate_one_process_use_case::CalculateOneProcessUseCase;
use crate::application::usecase::interface::AbstractUseCase;
use async_graphql::{Context, Object};
use sea_orm::QueryFilter;
use sea_orm::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;
use ulid::Ulid;

use crate::adapters::external::grpc::process_service_client::GrpcProcessServiceClient;
use crate::adapters::repository::mysql::entity as db_entity;

pub struct Mutation {
    db: DatabaseConnection,
}

impl Mutation {
    pub fn new(db: DatabaseConnection) -> Self {
        Mutation { db }
    }
}

fn create_step(step_input: &CreateStepInput, id: String) -> Step {
    Step {
        id,
        description: step_input.description.clone(),
        resource_id: step_input.resource_id,
        order_number: step_input.order_number,
        duration: step_input.duration,
    }
}
#[Object]
impl Mutation {
    async fn create_recipe_detail(
        &self,
        _ctx: &Context<'_>,
        recipe_detail_data: CreateRecipeDetailInput,
    ) -> Result<RecipeDetail, String> {
        let recipe_id = Ulid::new().to_string();
        let recipe = db_entity::recipes::ActiveModel {
            id: Set(recipe_id.clone()),
            title: Set(recipe_detail_data.title.clone()),
            description: Set(Some(recipe_detail_data.description.clone())),
        };
        recipe.insert(&self.db).await.unwrap();

        let steps: Vec<Step> = recipe_detail_data
            .steps
            .into_iter()
            .map(|step| create_step(&step, Ulid::new().to_string()))
            .collect();
        let step_models: Vec<db_entity::steps::ActiveModel> = steps
            .iter()
            .map(|step| db_entity::steps::ActiveModel {
                id: Set(step.id.clone()),
                recipe_id: Set(recipe_id.clone()),
                description: Set(step.description.clone()),
                resource_id: Set(step.resource_id),
                order_number: Set(step.order_number),
                duration: Set(step.duration),
            })
            .collect();
        db_entity::steps::Entity::insert_many(step_models)
            .exec(&self.db)
            .await
            .unwrap();

        let recipe_detail = RecipeDetail {
            id: recipe_id.clone(),
            title: recipe_detail_data.title,
            description: recipe_detail_data.description,
            steps,
        };

        Ok(recipe_detail)
    }

    async fn update_recipe_detail(
        &self,
        _ctx: &Context<'_>,
        recipe_detail_data: RecipeDetail,
    ) -> Result<RecipeDetail, String> {
        let recipe_model: db_entity::recipes::Model =
            db_entity::recipes::Entity::find_by_id(recipe_detail_data.id.clone())
                .one(&self.db)
                .await
                .unwrap()
                .unwrap();
        let mut recipe: db_entity::recipes::ActiveModel = recipe_model.into();
        recipe.title = Set(recipe_detail_data.title.clone());
        recipe.description = Set(Some(recipe_detail_data.description.clone()));
        let _updated_recipe = recipe.update(&self.db).await.unwrap();

        let _ = db_entity::steps::Entity::delete_many()
            .filter(db_entity::steps::Column::RecipeId.eq(recipe_detail_data.id.clone()))
            .exec(&self.db)
            .await
            .unwrap();
        let steps: Vec<db_entity::steps::ActiveModel> = recipe_detail_data
            .steps
            .iter()
            .map(|step| db_entity::steps::ActiveModel {
                id: Set(step.id.clone()),
                recipe_id: Set(recipe_detail_data.id.clone()),
                description: Set(step.description.clone()),
                resource_id: Set(step.resource_id),
                order_number: Set(step.order_number),
                duration: Set(step.duration),
            })
            .collect();
        let _ = db_entity::steps::Entity::insert_many(steps)
            .exec(&self.db)
            .await
            .unwrap();

        Ok(recipe_detail_data)
    }

    async fn create_resource(
        &self,
        _ctx: &Context<'_>,
        resource_data: CreateResourceInput,
    ) -> Result<Resource, String> {
        let resource = db_entity::resources::ActiveModel {
            id: NotSet,
            name: Set(resource_data.name.clone()),
            amount: Set(resource_data.amount),
        };
        let res = resource.insert(&self.db).await.unwrap();

        let resource = Resource {
            id: res.id,
            name: resource_data.name,
            amount: resource_data.amount,
        };

        Ok(resource)
    }

    async fn update_resource(
        &self,
        _ctx: &Context<'_>,
        resource_data: UpdateResourceInput,
    ) -> Result<Resource, String> {
        let _existing_resource: db_entity::resources::Model =
            db_entity::resources::Entity::find_by_id(resource_data.id)
                .one(&self.db)
                .await
                .unwrap()
                .ok_or_else(|| "Resource not found".to_string())?;
        let mut resource: db_entity::resources::ActiveModel = _existing_resource.into();
        resource.name = Set(resource_data.name.clone());
        resource.amount = Set(resource_data.amount);
        let _updated_resource = resource.update(&self.db).await.unwrap();
        let resource = Resource {
            id: resource_data.id,
            name: resource_data.name,
            amount: resource_data.amount,
        };

        Ok(resource)
    }

    async fn create_process(
        &self,
        ctx: &Context<'_>,
        recipe_id_list: CreateProcessInput,
    ) -> Result<ProcessId, String> {
        let process_repository = ctx
            .data::<Arc<MysqlProcessRepository>>()
            .map_err(|_| "Repository not found".to_string())?;
        let recipe_repository = ctx
            .data::<Arc<MysqlRecipeRepository>>()
            .map_err(|_| "Repository not found".to_string())?;
        let resource_repository = ctx
            .data::<Arc<MysqlResourceRepository>>()
            .map_err(|_| "Repository not found".to_string())?;
        let service = ctx
            .data::<Arc<GrpcProcessServiceClient>>()
            .map_err(|_| "Service not found".to_string())?;

        let use_case = CalculateOneProcessUseCase::new(
            process_repository.as_ref(),
            &recipe_id_list.recipe_id_list,
            service.as_ref(),
            recipe_repository.as_ref(),
            resource_repository.as_ref(),
        );
        let _process_id = use_case.execute().await.unwrap();

        Ok(ProcessId { id: 123 })
    }
}
