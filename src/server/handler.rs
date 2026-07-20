use crate::domain::conversion::record_to_prometheus_text;
use crate::domain::model::{AppState, GarniRecord};
use axum::http::{HeaderMap, header};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use std::time::SystemTime;

#[debug_handler]
pub async fn weather_set_handler(
    Query(gr): Query<GarniRecord>,
    State(state): State<AppState>,
    // State(db): State<DB1>,
) -> impl IntoResponse {
    let now = SystemTime::now();

    println!("Got data {:?}", now);
    let _ = state.write().unwrap().db.insert(gr);

    StatusCode::OK
}

#[debug_handler]
pub async fn prometheus_list_v004_handler(
    // Extension(state): Extension<SharedState>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let result = match &state.read().unwrap().db {
        None => "".to_string(),
        Some(gr) => record_to_prometheus_text(gr.clone()),
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/plain; version=0.0.4".parse().unwrap(),
    );
    (headers, result)
}

// pub async fn prometheus_list_v1_handler() -> impl IntoResponse {
//
//     String::from ("ela hop").into_response()
//
//     let mut headers = HeaderMap::new();
//     headers.insert(header::CONTENT_TYPE, "text/plain;version=1.0.0;escaping=allow-utf-8".parse().unwrap());
//     (headers, "foo")
// }
//
// pub async fn openmetrics_list_v1_handler() -> impl IntoResponse {
//
//     String::from ("ela hop").into_response()
//
//     let mut headers = HeaderMap::new();
//     headers.insert(header::CONTENT_TYPE, "application/openmetrics-text;version=1.0.0".parse().unwrap());
//     (headers, "foo")
// }
//
