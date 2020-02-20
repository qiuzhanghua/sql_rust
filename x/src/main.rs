use sqlx;
use sqlx::{MySqlPool, Pool};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //    let pool: PgPool = Pool::new(&db_url).await?;
    let pool: MySqlPool = Pool::new(&db_url).await?;
    let mut pool = pool.clone();
    let recs = sqlx::query!(r#"SELECT * from people"#)
        .fetch_all(&mut pool)
        .await
        .unwrap();
    for rec in recs {
        println!("{:?}", rec);
    }

    let person = sqlx::query_as!(Person, r#"SELECT * from people"#)
        .fetch_one(&mut pool)
        .await?;
    println!("{:?}", person);
    Ok(())
}

#[derive(Debug)]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub enabled: i8,
}
