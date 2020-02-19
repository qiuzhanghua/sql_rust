use sqlx;
use sqlx::{PgPool, MySqlPool, Pool};

// from shell run:
//  DATABASE_URL="mysql://app:app@localhost:3306/app" cargo run

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
//    let pool: PgPool = Pool::new(&db_url).await?;
    let pool: MySqlPool = Pool::new(&db_url).await?;
    let mut pool = pool.clone();
    let recs = sqlx::query!(r#"SELECT * from people limit 1"#)
        .fetch_all(&mut pool).await.unwrap();
    for rec in recs {
        println!("{:?}", rec.person_enabled == 1);
        println!("{:?}", rec);
    };

    let person = sqlx::query_as!(Person, r#"SELECT * from people"#)
        .fetch_all(&mut pool).await?;
    println!("{:?}", person);

    Ok(())
}

#[derive(Debug)]
pub struct Person {
    pub person_id: i64,
    pub person_name: String,
    pub person_email: String,
    pub person_enabled: i8,
}

