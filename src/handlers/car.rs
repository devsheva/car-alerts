use std::fs;

use axum::{extract::Path, response::IntoResponse, Json};
use chrono::NaiveDate;
use hyper::StatusCode;
use serde::Serialize;

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
    if fs::write(unsafe { FILE_PATH }, "[]").is_ok() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Serialize)]
pub struct NextRevisionDTO {
    plate: String,
    brand: Option<String>,
    next_revision: chrono::NaiveDate,
}

pub async fn next_revision(Path(plate): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    match Store::find_by_plate(plate.as_str()) {
        Some(index) => {
            let cars = Store::load();
            let car = &cars[index];
            let next_revision = car.last_revision + chrono::Months::new(24);

            Ok(Json(NextRevisionDTO {
                plate: car.plate.clone(),
                brand: car.brand.clone(),
                next_revision: next_revision.clone(),
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Serialize)]
pub struct NextRoadTaxDTO {
    plate: String,
    next_road_tax_date: NaiveDate,
}

pub async fn next_road_tax(Path(plate): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let cars = Store::load();

    match Store::find_by_plate(plate.as_str()) {
        Some(index) => {
            let car = &cars[index];
            let next_road_tax_date = car.last_road_tax + chrono::Months::new(12);

            Ok(Json(NextRoadTaxDTO {
                plate: car.plate.clone(),
                next_road_tax_date,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn mark_revision(Path(plate): Path<String>) -> StatusCode {
    let mut cars = Store::load();

    let car_index = Store::find_by_plate(plate.as_str());

    match car_index {
        Some(index) => {
            let car = cars.get_mut(index).unwrap();

            car.last_revision = chrono::Local::now().date_naive();

            Store::save(cars.as_slice());
            StatusCode::OK
        }
        None => StatusCode::NOT_FOUND,
    }
}

#[derive(Serialize)]
pub struct ChecklistDTO {
    plate: String,
    next_revision: NaiveDate,
    next_road_tax: NaiveDate,
}

pub async fn checklist(Path(plate): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let car_index = Store::find_by_plate(plate.as_str());

    match car_index {
        Some(index) => {
            let cars = Store::load();
            let car = &cars[index];

            let next_revision: NaiveDate = car.last_revision + chrono::Months::new(24);

            let next_road_tax: NaiveDate = car.last_road_tax + chrono::Months::new(12);

            Ok(Json(ChecklistDTO {
                plate: plate.clone(),
                next_revision,
                next_road_tax,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::utils::setup, router::app};
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_list_empty() {
        setup();
        let app = app();
        let server = TestServer::new(app).unwrap();

        let response = server.get("/cars").await;

        response.assert_status_ok();
        response.assert_text("[]");
    }

    // #[test]
    // fn test_add() {
    //     let add = Car {
    //         owner: "Mateo".to_string(),
    //         plate: "1234ABC".to_string(),
    //         brand: Some("Toyota".to_string()),
    //         last_revision: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
    //         last_road_tax: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
    //     };

    //     assert_eq!(add.owner, "Mateo");
    //     assert_eq!(add.plate, "1234ABC");
    //     assert_eq!(add.brand.unwrap(), "Toyota");

    //     teardown();
    // }

    // #[test]
    // fn test_add_duplicate_plate() {
    //     let car = Car {
    //         owner: "Mateo".to_string(),
    //         plate: "1234ABC".to_string(),
    //         brand: Some("Toyota".to_string()),
    //         last_revision: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
    //         last_road_tax: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
    //     };

    //     add(Json(car));

    //     let car = Car {
    //         owner: "Mateo".to_string(),
    //         plate: "1234ABC".to_string(),
    //         brand: Some("Toyota".to_string()),
    //         last_revision: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
    //         last_road_tax: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
    //     };

    //     let result = add(Json(car));

    //     assert_eq!(result.unwrap_err(), "Plate already exists");

    //     teardown();
    // }

    // #[tokio::test]
    // async fn test_reset() {
    //     setup();

    //     let content = fs::read_to_string(FILE_PATH).unwrap();
    //     assert_ne!(content, "[]");

    //     let result = reset().await;

    //     assert_eq!(result, StatusCode::OK);
    // }

    // #[tokio::test]
    // async fn test_next_revision() {
    //     setup();

    //     let cmd = NextRevision {
    //         plate: "1234ABC".to_string(),
    //     };

    //     let result = cmd.call().unwrap();
    //     assert_eq!(result.plate, "1234ABC");

    //     teardown();
    // }

    // #[tokio::test]
    // async fn test_not_found() {
    //     let cmd = NextRevision {
    //         plate: "1234ABC".to_string(),
    //     };

    //     let output = cmd.call();
    //     assert_eq!(output.unwrap_err(), "Car not found");
    // }

    // #[tokio::test]
    // async fn test_not_found() {
    //     let cmd = NextRoadTax {
    //         plate: "not_found".to_string(),
    //     };

    //     let result = cmd.call();
    //     assert_eq!(result.unwrap_err(), "Car with plate not_found not found");
    // }

    // #[tokio::test]
    // async fn test_success() {
    //     setup();

    //     let cmd = NextRoadTax {
    //         plate: "1234".to_string(),
    //     };

    //     let result = cmd.call();
    //     assert_eq!(
    //         result.unwrap().next_road_tax_date,
    //         NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
    //     );
    //     teardown();
    // }

    // #[test]
    // fn test_success() {
    //     setup();

    //     let cmd = MarkRevision {
    //         plate: "1234ABC".to_string(),
    //     };
    //     let result = cmd.call();

    //     assert_eq!(result.unwrap().done, true);

    //     let cars = Store::load();
    //     let car = cars.iter().find(|car| car.plate == "1234ABC").unwrap();
    //     assert_eq!(car.last_revision, chrono::Local::now().date_naive());

    //     teardown();
    // }

    // #[test]
    // fn test_not_found() {
    //     let cmd = MarkRevision {
    //         plate: "not-found".to_string(),
    //     };
    //     let result = cmd.call();
    //     assert!(result.is_err());
    // }

    // #[test]
    // fn test_success() {
    //     setup();
    //     let cmd = Checklist {
    //         plate: "123".to_string(),
    //     };

    //     let res = cmd.call();
    //     assert!(res.is_ok());

    //     teardown();
    // }

    // #[test]
    // fn test_failure() {
    //     let cmd = Checklist {
    //         plate: "1234".to_string(),
    //     };

    //     let res = cmd.call();
    //     assert_eq!(res.unwrap_err(), "Plate is invalid");
    // }
}
