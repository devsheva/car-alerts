use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use crate::store::{Car, Store};

pub async fn list() -> Json<Vec<Car>> {
    Json(Store::load())
}

pub async fn add(Json(car): Json<Car>) -> impl IntoResponse {
    let mut cars = Store::load();

    if let Some(_) = Store::find_by_plate(car.plate.as_str()) {
        return (StatusCode::NOT_FOUND, "Plate already exists".to_string());
    }

    cars.push(car.clone());

    Store::save(&cars);

    (StatusCode::CREATED, "Car added successfully".to_string())
}

#[cfg(test)]
mod tests {
    use crate::core::teardown;
    use chrono::NaiveDate;

    use super::*;

    #[tokio::test]
    async fn test_empty() {
        let result = list().await;

        assert_eq!(result.len(), 0)
    }

    #[test]
    fn test_add() {
        let add = Car {
            owner: "Mateo".to_string(),
            plate: "1234ABC".to_string(),
            brand: Some("Toyota".to_string()),
            last_revision: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
            last_road_tax: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
        };

        assert_eq!(add.owner, "Mateo");
        assert_eq!(add.plate, "1234ABC");
        assert_eq!(add.brand.unwrap(), "Toyota");

        teardown();
    }

    #[test]
    fn test_add_duplicate_plate() {
        let car = Car {
            owner: "Mateo".to_string(),
            plate: "1234ABC".to_string(),
            brand: Some("Toyota".to_string()),
            last_revision: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
            last_road_tax: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
        };

        add(Json(car));

        let car = Car {
            owner: "Mateo".to_string(),
            plate: "1234ABC".to_string(),
            brand: Some("Toyota".to_string()),
            last_revision: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
            last_road_tax: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
        };

        let result = add(Json(car));

        assert_eq!(result.unwrap_err(), "Plate already exists");

        teardown();
    }
}
