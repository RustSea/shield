use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use chrono::Utc;
use entity::api_user;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    mappers::{
        client::api_user::{CreateApiUserRequest, CreateApiUserResponse, UpdateApiUserRequest, UpdateApiUserResponse},
        DeleteResponse,
    },
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        token_user::TokenUser,
    },
    utils::{
        helpers::generate_random_string::{generate_random_string, Length},
        role_checker::{is_current_realm_admin, is_master_realm_admin},
    },
};

pub async fn get_api_users(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<api_user::Model>>, Error> {
    if !is_master_realm_admin(&user) && !is_current_realm_admin(&user, &realm_id.to_string()) {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    let api_users = api_user::Entity::find()
        .filter(api_user::Column::RealmId.eq(realm_id))
        .filter(api_user::Column::ClientId.eq(client_id))
        .all(&state.db)
        .await?;
    Ok(Json(api_users))
}

pub async fn create_api_user(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateApiUserRequest>,
) -> Result<Json<CreateApiUserResponse>, Error> {
    if !is_master_realm_admin(&user) && !is_current_realm_admin(&user, &realm_id.to_string()) {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    let api_secret = generate_random_string(Length::U64);

    let api_user_model = api_user::ActiveModel {
        id: Set(Uuid::now_v7()),
        secret: Set(api_secret),
        name: Set(payload.name),
        description: Set(payload.description),
        realm_id: Set(realm_id),
        client_id: Set(client_id),
        role: Set(payload.role),
        access: Set(payload.access),
        expires: Set(payload.expires.unwrap().to_datetime()),
        created_by: Set(user.sub),
        updated_by: Set(user.sub),
        ..Default::default()
    };

    let api_user = api_user_model.insert(&state.db).await?;
    Ok(Json(CreateApiUserResponse::from(api_user)))
}

pub async fn update_api_user(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, _client_id, api_user_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(payload): Json<UpdateApiUserRequest>,
) -> Result<Json<UpdateApiUserResponse>, Error> {
    if !is_master_realm_admin(&user) && !is_current_realm_admin(&user, &realm_id.to_string()) {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    let api_user = api_user::Entity::find_by_id(api_user_id).one(&state.db).await?;
    if api_user.is_none() {
        return Err(Error::not_found());
    }

    let api_user = api_user.unwrap();
    let api_user_model = api_user::ActiveModel {
        id: Set(api_user.id),
        realm_id: Set(api_user.realm_id),
        client_id: Set(api_user.client_id),
        name: Set(match payload.name {
            Some(name) => name,
            None => api_user.name,
        }),
        description: Set(match payload.description {
            Some(description) => Some(description),
            None => api_user.description,
        }),
        role: Set(match payload.role {
            Some(role) => role,
            None => api_user.role,
        }),
        access: Set(match payload.access {
            Some(access) => access,
            None => api_user.access,
        }),
        expires: Set(match payload.expires {
            Some(expires) => expires.to_datetime(),
            None => api_user.expires,
        }),
        locked_at: Set(match payload.lock {
            Some(true) => Some(Utc::now().into()),
            Some(false) => None,
            None => api_user.locked_at,
        }),
        ..Default::default()
    };

    let api_user = api_user_model.update(&state.db).await?;
    Ok(Json(UpdateApiUserResponse::from(api_user)))
}

pub async fn delete_api_user(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, _client_id, api_user_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<DeleteResponse>, Error> {
    if !is_master_realm_admin(&user) && !is_current_realm_admin(&user, &realm_id.to_string()) {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    let delete_result = api_user::Entity::delete_by_id(api_user_id).exec(&state.db).await?;
    Ok(Json(DeleteResponse {
        ok: delete_result.rows_affected == 1,
    }))
}