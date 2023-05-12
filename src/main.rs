use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Result, Surreal};

#[tokio::main]
async fn main() -> Result<()> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;

    let sql =
        std::fs::read_to_string("mock_data.sql").expect("Error reading from file mock_data.sql.");

    let num_inserts = 200;

    println!("Inserting...");
    for i in 0..50 {
        let sql = sql.clone();
        println!("{} - {}", i * num_inserts, (i + 1) * num_inserts);
        db.query(sql.clone())
            .await
            .expect("Error inserting into database.");
    }

    Ok(())
}
