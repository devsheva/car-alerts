use axum::Json;

use crate::store::{Car, Store};

pub async fn list() -> Json<Vec<Car>> {
    Json(Store::load())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_empty() {
        let result = list().await;

        assert_eq!(result.len(), 0)
    }
}
