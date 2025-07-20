use crate::handlers::car;
use axum::{
    routing::{delete, get},
    Router,
};

pub fn app() -> Router {
    Router::new()
        .route("/cars", get(car::list).post(car::add))
        .route("/cars/reset", delete(car::reset))
}

#[cfg(test)]
mod tests {}
