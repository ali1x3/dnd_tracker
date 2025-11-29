use anyhow::Error;
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;

pub async fn init_db() -> Result<(), Error> {
    let db = Surreal::new::<RocksDb>("db").await?;

    db.use_ns("app").use_db("main").await?;

    let response: Vec<serde_json::Value> = db.query("RETURN 123").await?.take(0)?;
    println!("Surreal test result: {:?}", response);

    Ok(())
}
