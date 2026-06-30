use axum::{Router, routing::get};

use crate::{
    handler::{health_checker_handler, prometheus_list_v004_handler, weather_set_handler},
    model,
};

pub fn create_router() -> Router {
    // let db = model::todo_db();
    // let db = model::create_db();
    // let db1 = model::create_db1();
    let app_state = model::create_app_state();

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/weatherstation/updateweatherstation.php",
            get(weather_set_handler),
        )
        .route("/weather/prometheus", get(prometheus_list_v004_handler))
        .with_state(app_state)
}
