use std::time::Duration;

use redis::Client;
use sqlx::{
    mysql::{MySql, MySqlPoolOptions},
    postgres::{PgPoolOptions, Postgres},
    Pool,
};

#[derive(Clone)]
pub struct AppState {
    pub redis_client: Client,
    pub counter: usize,
    pub mysql_client: Option<Pool<MySql>>,
    pub pgsql_client: Option<Pool<Postgres>>,
}

pub async fn new() -> AppState {
    // let mysql_client = MySqlPoolOptions::new()
    //     .max_connections(100)
    //     .min_connections(4)
    //     .acquire_timeout(Duration::from_secs(5))
    //     .max_lifetime(Duration::from_secs(60 * 60))
    //     .idle_timeout(Duration::from_secs(60 * 10))
    //     .connect("mysql://root:jjdev123@100.64.15.125/zcg")
    //     .await
    //     .unwrap();

    let pgsql_client = PgPoolOptions::new()
        .max_connections(100)
        .min_connections(4)
        .acquire_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(60 * 60))
        .idle_timeout(Duration::from_secs(60 * 10))
        .connect("postgres://postgres:zhang@localhost/test")
        .await
        .unwrap();

    AppState {
        redis_client: redis::Client::open("redis://127.0.0.1/").unwrap(),
        counter: 0,
        mysql_client: None,
        pgsql_client: Some(pgsql_client),
    }
}
