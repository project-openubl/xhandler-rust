use std::fmt::{Debug, Formatter};

use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use ublhub_entity::project;

use crate::db::Transactional;
use crate::system::error::Error;
use crate::system::InnerSystem;

impl InnerSystem {
    pub async fn create_project(
        &self,
        model: &project::Model,
        tx: Transactional<'_>,
    ) -> Result<ProjectContext, Error> {
        if let Some(found) = self.get_project(&model.name, tx).await? {
            Ok(found)
        } else {
            let entity = project::ActiveModel {
                name: Set(model.name.clone()),
                description: Set(model.description.clone()),
                sunat_username: Set(model.sunat_username.clone()),
                sunat_password: Set(model.sunat_password.clone()),
                sunat_factura_url: Set(model.sunat_factura_url.clone()),
                sunat_guia_url: Set(model.sunat_guia_url.clone()),
                sunat_percepcion_retencion_url: Set(model.sunat_percepcion_retencion_url.clone()),
            };

            Ok((self, entity.insert(&self.connection(tx)).await?).into())
        }
    }

    pub async fn get_project(
        &self,
        name: &str,
        tx: Transactional<'_>,
    ) -> Result<Option<ProjectContext>, Error> {
        Ok(project::Entity::find()
            .filter(project::Column::Name.eq(name))
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
    // pub async fn set_owner(&self, tx: Transactional<'_>) -> Result<Vec<ProjectContext>, Error> {
    //     Ok(advisory::Entity::find()
    //         .join(
    //             JoinType::Join,
    //             advisory_vulnerability::Relation::Advisory.def().rev(),
    //         )
    //         .filter(advisory_vulnerability::Column::VulnerabilityId.eq(self.cve.id))
    //         .all(&self.system.connection(tx))
    //         .await?
    //         .drain(0..)
    //         .map(|advisory| (&self.system, advisory).into())
    //         .collect())
    // }
}
