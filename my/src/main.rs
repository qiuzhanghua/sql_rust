#[macro_use]
extern crate mysql;

use r2d2;
use r2d2_mysql;

use r2d2_mysql::mysql::{Opts, OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() {
    // See docs on the `OptsBuilder`'s methods for the list of options available via URL.
    //    let pool = my::Pool::new("mysql://app:app123@localhost:3306/app").unwrap();

    let db_url = "mysql://app:app123@localhost:3306/app";
    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

    // Let's create payment table.
    // Unwrap just to make sure no error happened.
    //    pool.prep_exec(r"CREATE TEMPORARY TABLE payment (
    //                         customer_id int not null,
    //                         amount int not null,
    //                         account_name text
    //                     )", ()).unwrap();

    let payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];

    let mut pool = pool.get().unwrap();
    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool
        .prepare(
            r"INSERT INTO payment
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (:customer_id, :amount, :account_name)",
        )
        .into_iter()
    {
        for p in payments.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params! {
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            })
            .unwrap();
        }
    }

    // Let's select payments from database
    let selected_payments: Vec<Payment> = pool
        .prep_exec("SELECT customer_id, amount, account_name from payment", ())
        .map(|result| {
            // In this closure we will map `QueryResult` to `Vec<Payment>`
            // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
            // will map each `MyResult` to contained `row`(no proper error handling)
            //                // and second call to `map` will map each `row` to `Payment`
            //                result.map(|x| x.unwrap()).map(|row (no proper error handling)
            // and second call to `map` will map each `row` to `Payment`
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow your schema
                    let (customer_id, amount, account_name) = mysql::from_row(row);
                    Payment {
                        customer_id,
                        amount,
                        account_name,
                    }
                })
                .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        })
        .unwrap(); // Unwrap `Vec<Payment>`

    // Now make sure that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows without `ORDER BY`
    // so assume we are lucky.
    assert_eq!(payments, selected_payments);
    println!("Yay!");
}
