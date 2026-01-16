use axum::{
    Json,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use common::{ReferralOwnerNew, ReferralRedemptionNew};
use csv::Writer;
use serde::Deserialize;
use serde_json::Value;

use crate::AppState;

#[derive(Deserialize)]
pub struct RedemptionPath {
    pub code: String,
    pub meta: String,
    pub ts: String,
}

fn favicon_no_content() -> Response {
    (
        [(header::CONTENT_TYPE, "image/x-icon")],
        StatusCode::NO_CONTENT,
    )
        .into_response()
}

pub async fn health_check() -> Response {
    (StatusCode::OK, "API is healthy").into_response()
}

pub async fn options_ok() -> Response {
    StatusCode::NO_CONTENT.into_response()
}

pub async fn export_project_csv(
    State(state): State<std::sync::Arc<AppState>>,
    Path(project): Path<String>,
) -> Response {
    let rows = match state.referral_service.get_project_export_rows(&project) {
        Ok(rows) => rows,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "export failed").into_response();
        }
    };

    export_csv_response(rows)
}

pub async fn export_bitdca_csv(State(state): State<std::sync::Arc<AppState>>) -> Response {
    let rows = match state.referral_service.get_project_export_rows("bitDCA") {
        Ok(rows) => rows,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "export failed").into_response();
        }
    };

    export_csv_response(rows)
}

fn export_csv_response(rows: Vec<database::referral_repo::ProjectExportRow>) -> Response {
    let mut writer = Writer::from_writer(Vec::new());
    if writer
        .write_record([
            "owner_id",
            "owner_meta",
            "owner_created_at",
            "owner_updated_at",
            "code",
            "code_is_active",
            "code_use_count",
            "code_created_at",
            "code_updated_at",
            "redemption_id",
            "redemption_meta",
            "redemption_created_at",
        ])
        .is_err()
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "export failed").into_response();
    }

    for row in rows {
        let owner_meta = serde_json::to_string(&row.owner_meta).unwrap_or_default();
        let redemption_meta = row
            .redemption_meta
            .as_ref()
            .and_then(|meta| serde_json::to_string(meta).ok())
            .unwrap_or_default();
        let redemption_id = row
            .redemption_id
            .map(|value| value.to_string())
            .unwrap_or_default();
        let redemption_created_at = row
            .redemption_created_at
            .map(|value| value.to_string())
            .unwrap_or_default();

        if writer
            .write_record([
                row.owner_id.to_string(),
                owner_meta,
                row.owner_created_at.to_string(),
                row.owner_updated_at.to_string(),
                row.code,
                row.code_is_active.to_string(),
                row.code_use_count.to_string(),
                row.code_created_at.to_string(),
                row.code_updated_at.to_string(),
                redemption_id,
                redemption_meta,
                redemption_created_at,
            ])
            .is_err()
        {
            return (StatusCode::INTERNAL_SERVER_ERROR, "export failed").into_response();
        }
    }

    let csv_data = match writer.into_inner() {
        Ok(data) => data,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "export failed").into_response(),
    };

    (
        [(header::CONTENT_TYPE, "text/csv; charset=utf-8")],
        csv_data,
    )
        .into_response()
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
    Path(path): Path<RedemptionPath>,
) -> Response {
    let RedemptionPath { code, meta, ts } = path;
    let code = code.trim().to_string();
    if code.is_empty() {
        return favicon_no_content();
    }
    let parsed_meta = serde_json::from_str::<Value>(&meta)
        .ok()
        .and_then(|value| match value {
            Value::Null => None,
            Value::Object(map) if !map.is_empty() => Some(Value::Object(map)),
            Value::Object(_) => None,
            other => Some(other),
        });

    tracing::info!("received redemption {}, {}, {:?}", ts, code, meta);

    let state = state.clone();
    tokio::spawn(async move {
        let payload = ReferralRedemptionNew {
            code,
            meta: parsed_meta,
        };
        if let Err(err) = state.referral_service.create_redemption(payload) {
            tracing::warn!("failed to create redemption: {:?}", err);
        }
    });

    favicon_no_content()
}
