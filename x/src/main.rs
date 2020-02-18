use sqlx;
use sqlx::{PgPool, MySqlPool, Pool};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
//    let pool: PgPool = Pool::new("postgres://app:app@localhost:5432/app").await?;
    let pool: MySqlPool = Pool::new("mysql://app:app@localhost:3306/app").await?;
    let mut pool = pool.clone();
    let recs = sqlx::query!(r#"SELECT * from people"#)
        .fetch_all(&mut pool).await.unwrap();
    for rec in recs {
        println!("{:?}", rec);
    }
    Ok(())
}


