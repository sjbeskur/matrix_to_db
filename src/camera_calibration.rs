use nalgebra::Matrix3;
use sqlx::{Pool, Postgres, FromRow, Type, Decode, Encode, postgres::{PgTypeInfo, PgValueRef}};
use tokio;

#[derive(Debug, FromRow)]
pub struct CameraCalibration {
    pub id: i32,
    pub camera: String,
    pub description: String,
    pub kmatrix: Vec<i32>, // Flat Vec<i32> for integer[3][3]
}


// #[derive(Debug)]
// struct Matrix3Wrapper(Vec<i32>); // Store as flat Vec<i32> internally

// impl Type<Postgres> for Matrix3Wrapper {
//     fn type_info() -> PgTypeInfo {
//         PgTypeInfo::with_name("INT4[]") // 2D integer array
//     }
// }

// impl<'r> Decode<'r, Postgres> for Matrix3Wrapper {
//     fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
//         // Decode the 2D array as Vec<Vec<i32>>
//         let vec_2d: Vec<Vec<i32>> = Vec::<Vec<i32>>::decode(value)?;
//         // Validate dimensions (3x3)
//         if vec_2d.len() != 3 || vec_2d.iter().any(|row| row.len() != 3) {
//             return Err("Expected a 3x3 array".into());
//         }
//         // Flatten to Vec<i32> (row-major order)
//         let flat: Vec<i32> = vec_2d.into_iter().flatten().collect();
//         Ok(Matrix3Wrapper(flat))
//     }
// }
