use nalgebra::Matrix3;
use sqlx::{Pool, Postgres};
use tokio;

use matrix_to_db::*;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to PostgreSQL
    let pool = Pool::<Postgres>::connect("postgres://admin:password123@localhost:6500/astrix_db")
        .await?;



    for i in  0..1000{
        // Example: Create a nalgebra Matrix3<i32>
        let matrix = Matrix3::new(1, 2, 3, 4, 5, 6, 7, 8, 9);

        // Insert a new calibration
        database::insert_calibration(&pool, i, "Vis", "Canon", matrix).await?;
        println!("Inserted calibration with id = 3");
    }

    for i in 0..100{
        // Fetch the calibration with Id = 3    
        let calibration = database::fetch_calibration(&pool, i).await?;

        println!("Retrieved calibration: {:?}", calibration);
        // Convert kmatrix to nalgebra Matrix3
        let matrix = database::kmatrix_to_nalgebra(&calibration.kmatrix)?;
        println!("nalgebra Matrix3:\n{}", matrix);
    }



    Ok(())
}