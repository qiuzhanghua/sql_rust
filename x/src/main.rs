use sqlx;
use sqlx::{PgPool, MySqlPool, Pool};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
//    let pool: PgPool = Pool::new(&db_url).await?;
    let pool: MySqlPool = Pool::new(&db_url).await?;
    let mut pool = pool.clone();
    let recs = sqlx::query!(r#"SELECT * from people"#)
        .fetch_all(&mut pool).await.unwrap();
    for rec in recs {
        println!("{:?}", rec);
    }
    Ok(())
}


