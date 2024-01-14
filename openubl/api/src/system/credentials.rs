use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

use openubl_entity as entity;

use crate::db::Transactional;
use crate::system::error::Error;
use crate::system::InnerSystem;

pub struct CredentialsContext {
    pub system: InnerSystem,
    pub credentials: entity::credentials::Model,
}

impl From<(&InnerSystem, entity::credentials::Model)> for CredentialsContext {
    fn from((system, credentials): (&InnerSystem, entity::credentials::Model)) -> Self {
        Self {
            system: system.clone(),
            credentials,
        }
    }
}

impl CredentialsContext {
    pub async fn update(
        &self,
        model: &entity::credentials::Model,
        tx: Transactional<'_>,
    ) -> Result<(), Error> {
        let mut entity: entity::credentials::ActiveModel = self.credentials.clone().into();

        entity.name = Set(model.name.clone());
        entity.username_sol = Set(model.username_sol.clone());
        entity.password_sol = Set(model.password_sol.clone());
        entity.client_id = Set(model.client_id.clone());
        entity.client_secret = Set(model.client_secret.clone());

        entity.update(&self.system.connection(tx)).await?;
        Ok(())
    }

    pub async fn delete(&self, tx: Transactional<'_>) -> Result<(), Error> {
        let entity: entity::credentials::ActiveModel = self.credentials.clone().into();
        entity.delete(&self.system.connection(tx)).await?;
        Ok(())
    }
}
