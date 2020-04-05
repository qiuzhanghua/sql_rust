use sqlx::types::time::PrimitiveDateTime;
use sqlx::{self, MySqlPool, PgPool, Pool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: PgPool = Pool::new(&db_url).await?;
    // let pool: MySqlPool = Pool::new(&db_url).await?;
    let recs = sqlx::query!(r#"SELECT * from people"#)
        .fetch_all(&mut &pool)
        .await
        .unwrap();
    for rec in recs {
        println!("{:?}", rec);
    }

    // let person = sqlx::query_as!(Person, r#"SELECT * from people"#)
    //     .fetch_one(&mut &pool)
    //     .await?;
    // println!("{:?}", person);
    Ok(())
}

#[derive(Debug)]
pub struct Person {
    pub person_id: String,
    pub person_name: String,
    pub person_pass: String,
    pub person_email: String,
    pub created: PrimitiveDateTime, // chrono::naive::datetime::NaiveDateTime for PG
    pub updated: PrimitiveDateTime, // chrono::naive::datetime::NaiveDateTime for PG
    pub person_enabled: i8,         //bool for PG
    pub dept_id: Option<String>,
}


// DATABASE_URL="postgres://app:app@localhost:5432/app" cargo run
// DATABASE_URL="mysql://app:app@localhost:3306/app" cargo run