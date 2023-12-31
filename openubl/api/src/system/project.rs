use std::fmt::{Debug, Formatter};

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, RelationTrait,
};
use sea_query::JoinType;

use openubl_entity as entity;
use openubl_entity::project;
use openubl_entity::user_role;

use crate::db::Transactional;
use crate::system::error::Error;
use crate::system::InnerSystem;

impl InnerSystem {
    pub async fn get_projects_by_user_id(
        &self,
        user_id: &str,
        tx: Transactional<'_>,
    ) -> Result<Vec<ProjectContext>, Error> {
        Ok(project::Entity::find()
            .join(
                JoinType::InnerJoin,
                user_role::Relation::Project.def().rev(),
            )
            .filter(entity::user_role::Column::UserId.eq(user_id))
            .all(&self.connection(tx))
            .await?
            .drain(0..)
            .map(|project| (self, project).into())
            .collect())
    }

    pub async fn create_project(
        &self,
        model: &project::Model,
        user_id: &str,
        tx: Transactional<'_>,
    ) -> Result<ProjectContext, Error> {
        let project_entity = project::ActiveModel {
            name: Set(model.name.clone()),
            description: Set(model.description.clone()),
            ..Default::default()
        };

        let result = project_entity.insert(&self.connection(tx)).await?;

        let user_role_entity = user_role::ActiveModel {
            project_id: Set(result.id),
            user_id: Set(user_id.to_string()),
            role: Set(user_role::Role::Owner),
        };
        user_role_entity.insert(&self.connection(tx)).await?;

        Ok((self, result).into())
    }

    pub async fn get_project(
        &self,
        id: i32,
        tx: Transactional<'_>,
    ) -> Result<Option<ProjectContext>, Error> {
        Ok(project::Entity::find_by_id(id)
            .one(&self.connection(tx))
            .await?
            .map(|project| (self, project).into()))
    }
}

#[derive(Clone)]
pub struct ProjectContext {
    pub system: InnerSystem,
    pub project: project::Model,
}

impl Debug for ProjectContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.project.fmt(f)
    }
}

impl From<(&InnerSystem, project::Model)> for ProjectContext {
    fn from((system, project): (&InnerSystem, project::Model)) -> Self {
        Self {
            system: system.clone(),
            project,
        }
    }
}

impl ProjectContext {
    pub async fn set_owner(&self, user_id: &str, tx: Transactional<'_>) -> Result<(), Error> {
        let entity = user_role::ActiveModel {
            project_id: Set(self.project.id),
            user_id: Set(user_id.to_string()),
            role: Set(user_role::Role::Owner),
        };
        entity.insert(&self.system.connection(tx)).await?;
        Ok(())
    }
}
