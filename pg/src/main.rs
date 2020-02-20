use postgres::{NoTls};
use r2d2_postgres::PostgresConnectionManager;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let manager = PostgresConnectionManager::new(
        "host=localhost user=app password=app123".parse().unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();

    let mut client = pool.get().unwrap();

    //    let mut client = Client::connect("host=localhost user=app password=app123", NoTls)?;

    //    client.batch_execute("
    //    CREATE TABLE person (
    //        id      SERIAL PRIMARY KEY,
    //        name    TEXT NOT NULL,
    //        data    BYTEA
    //    )
    //")?;

    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    )?;

    for row in client.query("SELECT id, name, data FROM person", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
    Ok(())
}
