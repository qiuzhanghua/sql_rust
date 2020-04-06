// use sqlx::types::time::PrimitiveDateTime;
use sqlx::prelude;
use sqlx::types::chrono;
use sqlx::{self, MySqlPool, PgPool, Pool};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
       let pool: PgPool = Pool::new(&db_url).await?;
//    let pool: MySqlPool = Pool::new(&db_url).await?;
    // let recs = sqlx::query!(r#"SELECT * from people"#)
    //     .fetch_all(&mut &pool)
    //     .await?;
    // for rec in recs {
    //     println!("{:?}", rec);
    // }
    //
    // let person = sqlx::query_as!(Person, r#"SELECT * from people"#)
    //     .fetch_one(&mut &pool)
    //     .await?;
    // println!("{:?}", person);

    // let pool: MySqlPool = Pool::new(&db_url).await.unwrap();
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    // use sqlx_core::query::MapRow;

    let mut cursor = sqlx::query(r#"SELECT * from people"#)
        .fetch(&pool);
     let row = cursor.next().await?.unwrap();
    println!("{:?}", row.get::<&str, &str>("person_id"));

    // let mut cursor2 = sqlx::query(r#"SELECT * from people"#)
    //     .map(|row:MySqlRow| row.get::<String, &str>("person_id"))
    //     .fetch_all(&pool).await.unwrap();
    // let row = cursor2;
    // println!("{:?}", cursor2);

    Ok(())
}

#[derive(Debug)]
pub struct Person {
    pub person_id: String,
    pub person_name: String,
    pub person_pass: String,
    pub person_email: String,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub person_enabled: i8, //bool for PG
    pub dept_id: Option<String>,
}

// DATABASE_URL="postgres://app:app@localhost:5432/app" cargo run
// DATABASE_URL="mysql://app:app@localhost:3306/app" cargo run
