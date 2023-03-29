use actix_web::web;

use crate::{app_state, errors::WebError};

pub async fn general_health_check(
    app_state: web::Data<app_state::AppState>,
) -> Result<String, WebError> {
    let mut visit_count = app_state.visit_count.lock().await;
    *visit_count += 1;
    Ok(format!("I've been seen {} times", *visit_count))
}
