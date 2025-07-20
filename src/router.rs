use crate::handlers::car;
use axum::{routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/cars", get(car::list))
}

#[cfg(test)]
mod tests {}
