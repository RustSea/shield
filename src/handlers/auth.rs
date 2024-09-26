use std::sync::Arc;

use crate::{
    database::{
        client,
        prelude::{Client, Resource, ResourceGroup, User},
        resource, resource_group, user,
    },
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        settings::SETTINGS,
        token::{create, Claims, TokenUser},
    },
};
use axum::{extract::Path, Extension, Json};
use sea_orm::{prelude::Uuid, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, RelationTrait};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<Credentials>,
) -> Result<Json<LoginResponse>, Error> {
    debug!("🚀 Login request received!");
    let user_with_resource_groups = User::find()
        .filter(user::Column::Email.eq(payload.email))
        .find_also_related(ResourceGroup)
        .filter(resource_group::Column::RealmId.eq(realm_id))
        .filter(resource_group::Column::ClientId.eq(client_id))
        .one(&state.db)
        .await?;

    if user_with_resource_groups.is_none() {
        debug!("No matching data found");
        return Err(Error::not_found());
    }

    let (user, resource_groups) = user_with_resource_groups.unwrap();
    if !user.verify_password(&payload.password) {
        debug!("Wrong password");
        return Err(Error::Authenticate(AuthenticateError::WrongCredentials));
    }
    if user.locked_at.is_some() {
        debug!("User is locked");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    if resource_groups.is_none() {
        debug!("No matching resource group found");
        return Err(Error::not_found());
    }

    let resource_groups = resource_groups.unwrap();
    if resource_groups.locked_at.is_some() {
        debug!("Resource group is locked");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    // Fetch client separately
    let client = Client::find_by_id(client_id).one(&state.db).await?.ok_or_else(|| {
        debug!("No client found");
        Error::not_found()
    })?;

    if client.locked_at.is_some() {
        debug!("Client is locked");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    // Fetch resources
    let resources = Resource::find()
        .filter(resource::Column::GroupId.eq(resource_groups.id))
        .filter(resource::Column::LockedAt.is_null())
        .all(&state.db)
        .await?;

    if resources.is_empty() {
        debug!("No resources found");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    let access_token = create(user.clone(), client, resource_groups, resources, &SETTINGS.secrets.signing_key).unwrap();
    Ok(Json(LoginResponse { access_token }))
}

pub async fn register(Extension(state): Extension<Arc<AppState>>) {
    debug!("🚀 Register request received!");

    todo!();
}

pub async fn verify() {
    debug!("🚀 Verify request received!");
    todo!();
}