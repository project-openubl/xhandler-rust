use std::fmt::{Debug, Formatter};

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};
use sea_query::JoinType;
use sea_query::Order::Desc;

use openubl_entity as entity;

use crate::db::{Paginated, PaginatedResults, Transactional};
use crate::system::credentials::CredentialsContext;
use crate::system::error::Error;
use crate::system::ubl_document::UblDocumentContext;
use crate::system::InnerSystem;

impl InnerSystem {
    pub async fn list_projects(
        &self,
        user_id: &str,
        tx: Transactional<'_>,
    ) -> Result<Vec<ProjectContext>, Error> {
        Ok(entity::project::Entity::find()
            .join(
                JoinType::InnerJoin,
                entity::user_role::Relation::Project.def().rev(),
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
        model: &entity::project::Model,
        user_id: &str,
        tx: Transactional<'_>,
    ) -> Result<ProjectContext, Error> {
        let project_entity = entity::project::ActiveModel {
            name: Set(model.name.clone()),
            description: Set(model.description.clone()),
            ..Default::default()
        };

        let result = project_entity.insert(&self.connection(tx)).await?;

        let user_role_entity = entity::user_role::ActiveModel {
            project_id: Set(result.id),
            user_id: Set(user_id.to_string()),
            role: Set(entity::user_role::Role::Owner),
        };
        user_role_entity.insert(&self.connection(tx)).await?;

        Ok((self, result).into())
    }

    pub async fn get_project(
        &self,
        id: i32,
        user_id: &str,
        tx: Transactional<'_>,
    ) -> Result<Option<ProjectContext>, Error> {
        Ok(entity::project::Entity::find_by_id(id)
            .join(
                JoinType::InnerJoin,
                entity::user_role::Relation::Project.def().rev(),
            )
            .filter(entity::user_role::Column::UserId.eq(user_id))
            .one(&self.connection(tx))
            .await?
            .map(|project| (self, project).into()))
    }
}

#[derive(Clone)]
pub struct ProjectContext {
    pub system: InnerSystem,
    pub project: entity::project::Model,
}

impl Debug for ProjectContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.project.fmt(f)
    }
}

impl From<(&InnerSystem, entity::project::Model)> for ProjectContext {
    fn from((system, project): (&InnerSystem, entity::project::Model)) -> Self {
        Self {
            system: system.clone(),
            project,
        }
    }
}

impl ProjectContext {
    pub async fn update(
        &self,
        model: &entity::project::Model,
        tx: Transactional<'_>,
    ) -> Result<(), Error> {
        let mut entity: entity::project::ActiveModel = self.project.clone().into();

        entity.name = Set(model.name.clone());
        entity.description = Set(model.description.clone());

        entity.update(&self.system.connection(tx)).await?;
        Ok(())
    }

    pub async fn delete(&self, tx: Transactional<'_>) -> Result<(), Error> {
        let entity: entity::project::ActiveModel = self.project.clone().into();
        entity.delete(&self.system.connection(tx)).await?;
        Ok(())
    }

    // Documents
    pub async fn get_document(
        &self,
        id: i32,
        tx: Transactional<'_>,
    ) -> Result<Option<UblDocumentContext>, Error> {
        Ok(entity::ubl_document::Entity::find()
            .join(
                JoinType::InnerJoin,
                entity::credentials::Relation::Project.def(),
            )
            .filter(entity::credentials::Column::Id.eq(id))
            .filter(entity::credentials::Column::ProjectId.eq(self.project.id))
            .one(&self.system.connection(tx))
            .await?
            .map(|e| (&self.system, e).into()))
    }

    pub async fn get_document_by_ubl_params(
        &self,
        ruc: &str,
        document_type: &str,
        document_id: &str,
        sha256: &str,
        tx: Transactional<'_>,
    ) -> Result<Option<UblDocumentContext>, Error> {
        Ok(entity::ubl_document::Entity::find()
            .filter(entity::ubl_document::Column::DocumentType.eq(document_type))
            .filter(entity::ubl_document::Column::DocumentId.eq(document_id))
            .filter(entity::ubl_document::Column::SupplierId.eq(ruc))
            .filter(entity::ubl_document::Column::Sha256.eq(sha256))
            .one(&self.system.connection(tx))
            .await?
            .map(|e| (&self.system, e).into()))
    }

    pub async fn create_document(
        &self,
        model: &entity::ubl_document::Model,
        tx: Transactional<'_>,
    ) -> Result<UblDocumentContext, Error> {
        let entity = entity::ubl_document::ActiveModel {
            project_id: Set(self.project.id),
            file_id: Set(model.file_id.clone()),
            document_type: Set(model.document_type.clone()),
            document_id: Set(model.document_id.clone()),
            supplier_id: Set(model.supplier_id.clone()),
            voided_document_code: Set(model.voided_document_code.clone()),
            digest_value: Set(model.digest_value.clone()),
            sha256: Set(model.sha256.clone()),
            ..Default::default()
        };

        let result = entity.insert(&self.system.connection(tx)).await?;
        Ok((&self.system, result).into())
    }

    pub async fn list_documents(
        &self,
        paginated: Paginated,
        tx: Transactional<'_>,
    ) -> Result<PaginatedResults<UblDocumentContext>, Error> {
        let connection = self.system.connection(tx);
        let pagination = entity::ubl_document::Entity::find()
            .join(
                JoinType::InnerJoin,
                entity::ubl_document::Relation::Project.def(),
            )
            .filter(entity::ubl_document::Column::ProjectId.eq(self.project.id))
            .order_by(entity::ubl_document::Column::Id, Desc)
            .paginate(&connection, paginated.page_size());

        Ok(PaginatedResults {
            items: pagination
                .fetch_page(paginated.page_number())
                .await?
                .drain(0..)
                .map(|advisory| (&self.system, advisory).into())
                .collect::<Vec<UblDocumentContext>>(),
            num_items: pagination.num_items().await?,
        })
    }

    // Credentials

    pub async fn get_credential(
        &self,
        id: i32,
        tx: Transactional<'_>,
    ) -> Result<Option<CredentialsContext>, Error> {
        Ok(entity::credentials::Entity::find()
            .join(
                JoinType::InnerJoin,
                entity::credentials::Relation::Project.def(),
            )
            .filter(entity::credentials::Column::Id.eq(id))
            .filter(entity::credentials::Column::ProjectId.eq(self.project.id))
            .one(&self.system.connection(tx))
            .await?
            .map(|e| (&self.system, e).into()))
    }

    pub async fn get_credential_for_supplier_id(
        &self,
        supplier_id: &str,
        tx: Transactional<'_>,
    ) -> Result<Option<CredentialsContext>, Error> {
        Ok(entity::credentials::Entity::find()
            .join(
                JoinType::InnerJoin,
                entity::credentials::Relation::Project.def(),
            )
            .join(
                JoinType::InnerJoin,
                entity::credentials::Relation::SendRule.def(),
            )
            .filter(entity::send_rule::Column::SupplierId.eq(supplier_id))
            .filter(entity::credentials::Column::ProjectId.eq(self.project.id))
            .one(&self.system.connection(tx))
            .await?
            .map(|e| (&self.system, e).into()))
    }

    pub async fn create_credentials(
        &self,
        model: &entity::credentials::Model,
        supplier_ids: &[String],
        tx: Transactional<'_>,
    ) -> Result<CredentialsContext, Error> {
        let entity = entity::credentials::ActiveModel {
            project_id: Set(self.project.id),
            name: Set(model.name.clone()),
            username_sol: Set(model.username_sol.clone()),
            password_sol: Set(model.password_sol.clone()),
            client_id: Set(model.client_id.clone()),
            client_secret: Set(model.client_secret.clone()),
            ..Default::default()
        };

        let result = entity.insert(&self.system.connection(tx)).await?;

        let rules = supplier_ids
            .iter()
            .map(|supplier_id| entity::send_rule::ActiveModel {
                supplier_id: Set(supplier_id.clone()),
                credentials_id: Set(result.id),
                project_id: Set(self.project.id),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        let _rules = entity::send_rule::Entity::insert_many(rules)
            .exec(&self.system.connection(tx))
            .await?;

        Ok((&self.system, result).into())
    }

    pub async fn list_credentials(
        &self,
        tx: Transactional<'_>,
    ) -> Result<Vec<CredentialsContext>, Error> {
        Ok(entity::credentials::Entity::find()
            .join(
                JoinType::InnerJoin,
                entity::credentials::Relation::Project.def(),
            )
            .filter(entity::credentials::Column::ProjectId.eq(self.project.id))
            .all(&self.system.connection(tx))
            .await?
            .drain(0..)
            .map(|credentials| (&self.system, credentials).into())
            .collect())
    }
}
