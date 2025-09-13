use crate::adapters::db::db_process_registration_repository::DbProcessRegistrationRepository;
use crate::application::usecase::calculate_one_process_use_case::CalculateOneProcessUseCase;
use crate::presentation::graphql::object::{
    CreateProcessInput, CreateResourceInput, CreateStepInput, ProcessId, Resource,
    UpdateResourceInput,
};
use async_graphql::{Context, Object};
use sea_orm::QueryFilter;
use sea_orm::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;
use ulid::Ulid;

use crate::application::usecase::interface::AbstractUseCase;

use super::object::{CreateRecipeDetailInput, RecipeDetail, Step};

use crate::infrastructure::mysql::entity as db_entity;
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
        let process = db_entity::processes::ActiveModel {
            id: NotSet,
            name: Set("process".to_string()),
        };
        let _res = process.insert(&self.db).await.unwrap();
        let process_id = _res.id;

        let repository = ctx
            .data::<Arc<DbProcessRegistrationRepository>>()
            .map_err(|_| "Repository not found".to_string())?;
        let usecase =
            CalculateOneProcessUseCase::new(repository.as_ref(), &recipe_id_list.recipe_id_list);
        let _result = usecase.execute().await.unwrap();
        let recipe_id_list: Vec<db_entity::process_regsitrations::ActiveModel> = recipe_id_list
            .recipe_id_list
            .iter()
            .map(|recipe_id| db_entity::process_regsitrations::ActiveModel {
                id: NotSet,
                process_id: Set(process_id),
                recipe_id: Set(recipe_id.clone()),
            })
            .collect();
        let _inserted = db_entity::process_regsitrations::Entity::insert_many(recipe_id_list)
            .exec(&self.db)
            .await
            .unwrap();

        Ok(ProcessId { id: process_id })
    }
}
