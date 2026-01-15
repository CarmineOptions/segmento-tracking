use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use common::{ReferralOwnerNew, ReferralRedemptionNew};
use serde::Deserialize;
use serde_json::Value;

use crate::AppState;

#[derive(Deserialize)]
pub struct RedemptionQuery {
    pub meta: Option<String>,
}

pub async fn health_check() -> Response {
    (StatusCode::OK, "API is healthy").into_response()
}

pub async fn options_ok() -> Response {
    StatusCode::NO_CONTENT.into_response()
}

pub async fn create_owner(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<ReferralOwnerNew>,
) -> Result<Response, Response> {
    if !payload.meta.is_object() {
        return Err((StatusCode::BAD_REQUEST, "owner meta must be an object").into_response());
    }
    if payload.meta.as_object().unwrap().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "owner meta object cannot be empty").into_response());
    }

    let payload_meta = payload.meta.clone();
    let result = state.referral_service.create_owner(payload);
    match result {
        Ok(owner_with_code) => Ok((StatusCode::OK, Json(owner_with_code)).into_response()),
        Err(_) => {
            let existing = state
                .referral_service
                .get_owner_with_code(payload_meta)
                .map_err(|_| {
                    (StatusCode::INTERNAL_SERVER_ERROR, "owner creation failed").into_response()
                })?;

            match existing {
                Some(owner_with_code) => {
                    Ok((StatusCode::OK, Json(owner_with_code)).into_response())
                }
                None => {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "owner creation failed")
                        .into_response())
                }
            }
        }
    }
}

pub async fn create_redemption(
    State(state): State<std::sync::Arc<AppState>>,
    Path(code): Path<String>,
    Query(query): Query<RedemptionQuery>,
) -> Response {
    let code = code.trim().to_string();
    if code.is_empty() {
        return StatusCode::NO_CONTENT.into_response();
    }

    let meta = query
        .meta
        .as_deref()
        .and_then(|raw| serde_json::from_str::<Value>(raw).ok())
        .and_then(|value| match value {
            Value::Object(map) if !map.is_empty() => Some(Value::Object(map)),
            _ => None,
        });

    let state = state.clone();
    tokio::spawn(async move {
        let payload = ReferralRedemptionNew { code, meta };
        if let Err(err) = state.referral_service.create_redemption(payload) {
            tracing::warn!("failed to create redemption: {:?}", err);
        }
    });

    StatusCode::NO_CONTENT.into_response()
}
