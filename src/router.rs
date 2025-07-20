use crate::handlers::car;
use axum::{
    routing::{get, post},
    Router,
};

pub fn app() -> Router {
    Router::new().route("/cars", get(car::list).post(car::add))
}

#[cfg(test)]
mod tests {}
