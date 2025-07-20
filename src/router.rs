use crate::handlers::car;
use axum::{
    routing::{delete, get},
    Router,
};

pub fn app() -> Router {
    Router::new()
        .route("/cars", get(car::list).post(car::add))
        .route("/cars/reset", delete(car::reset))
        .route("/cars/{plate}/next_revision", get(car::next_revision))
}

#[cfg(test)]
mod tests {}
