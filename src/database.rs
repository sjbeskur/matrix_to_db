use nalgebra::Matrix3;
use sqlx::{Pool, Postgres};
use crate::camera_calibration::CameraCalibration;

pub async fn fetch_calibration(pool: &Pool<Postgres>, id: i32) -> Result<CameraCalibration, sqlx::Error> {
    sqlx::query_as::<_, CameraCalibration>(
        "SELECT id, camera, description, kmatrix FROM Camera_Calibrations WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await
}


// Insert a new calibration
pub async fn insert_calibration(
    pool: &Pool<Postgres>,
    id: i32,
    camera: &str,
    description: &str,
    kmatrix: Matrix3<i32>,
) -> Result<(), sqlx::Error> {
    let kmatrix_vec = matrix_to_vec(&kmatrix);
    sqlx::query(
        "INSERT INTO Camera_Calibrations (id, camera, description, kmatrix) VALUES ($1, $2, $3, $4)"
    )
    .bind(id)
    .bind(camera)
    .bind(description)
    .bind(kmatrix_vec)
    .execute(pool)
    .await?;
    Ok(())
}

// Update an existing calibration's kmatrix (and optionally other fields)
pub async fn update_calibration(
    pool: &Pool<Postgres>,
    id: i32,
    kmatrix: Matrix3<i32>,
    camera: Option<&str>, // Optional: update camera if provided
    description: Option<&str>, // Optional: update description if provided
) -> Result<(), sqlx::Error> {
    let kmatrix_vec = matrix_to_vec(&kmatrix);
    sqlx::query(
        "UPDATE Camera_Calibrations SET kmatrix = $1, camera = COALESCE($2, camera), description = COALESCE($3, description) WHERE id = $4"
    )
    .bind(kmatrix_vec)
    .bind(camera)
    .bind(description)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}



pub fn kmatrix_to_nalgebra(kmatrix: &Vec<i32>) -> Result<Matrix3<i32>, &'static str> {
    // Ensure kmatrix has 9 elements (3x3)
    if kmatrix.len() != 9 {
        return Err("kmatrix must have exactly 9 elements for a 3x3 matrix");
    }

    // Convert to nalgebra Matrix3
    Ok(Matrix3::new(
        kmatrix[0], kmatrix[1], kmatrix[2],
        kmatrix[3], kmatrix[4], kmatrix[5],
        kmatrix[6], kmatrix[7], kmatrix[8],
    ))
}

// Convert nalgebra Matrix3<i32> to Vec<i32> (row-major order)
pub fn matrix_to_vec(matrix: &Matrix3<i32>) -> Vec<i32> {
    vec![
        matrix[(0, 0)], matrix[(0, 1)], matrix[(0, 2)],
        matrix[(1, 0)], matrix[(1, 1)], matrix[(1, 2)],
        matrix[(2, 0)], matrix[(2, 1)], matrix[(2, 2)],
    ]
}



