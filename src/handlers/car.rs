use std::fs;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use crate::{
    core::FILE_PATH,
    store::{Car, Store},
};

pub async fn list() -> Json<Vec<Car>> {
    Json(Store::load())
}

pub async fn add(Json(car): Json<Car>) -> Result<impl IntoResponse, StatusCode> {
    let mut cars = Store::load();

    if let Some(_) = Store::find_by_plate(car.plate.as_str()) {
        return Err(StatusCode::NOT_FOUND);
    }

    cars.push(car.clone());

    Store::save(&cars);

    Ok((StatusCode::CREATED, Json(car)))
}

pub async fn reset() -> StatusCode {
    if fs::write(FILE_PATH, "[]").is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
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

    #[tokio::test]
    async fn test_reset() {
        setup();

        let content = fs::read_to_string(FILE_PATH).unwrap();
        assert_ne!(content, "[]");

        let result = reset().await;

        assert_eq!(result, StatusCode::OK);
    }
}
