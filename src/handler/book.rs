use std::{fmt::Debug, sync::Arc};

use axum::{
    Json, Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
// use sqlx::{mysql::MySqlRow, Row};
use sqlx::{
    Row,
    postgres::{PgQueryResult, PgRow},
};

use crate::state::AppState;
use crate::util;

pub fn router() -> Router<Arc<AppState>> {
    // <Arc<AppState>>
    Router::new().route("/", get(root)).route("/add", get(add))
}

async fn root(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let rows: Vec<PgRow> = sqlx::query("SELECT * FROM t2;")
        .fetch_all(&app_state.pgsql_client.clone().unwrap())
        .await
        .unwrap();

    #[derive(Serialize, Default, Clone)]
    struct User {
        id: i32,
        name: String,
        age: i32,
        sex: String,
    }

    let mut users = vec![];

    for row in rows.iter() {
        let user = User {
            id: row.get(0),
            name: row.get(1),
            age: row.get(2),
            sex: row.get(3),
        };

        users.push(user);
    }

    let data = util::Data::<Vec<User>>::new();

    Json(data.ok(users))
}

#[derive(Deserialize)]
pub struct AddOneReq {
    name: String,
    age: i32,
    sex: String,
}

async fn add(State(app_state): State<Arc<AppState>>, args: Query<AddOneReq>) -> impl IntoResponse {
    let rows: Vec<PgRow> = sqlx::query("SELECT * FROM t2;")
        .fetch_all(&app_state.pgsql_client.clone().unwrap())
        .await
        .unwrap();

    let result = sqlx::query("INSERT INTO t2 (id, name, age, sex) VALUES ($1, $2, $3, $4);")
        .bind(100)
        .bind(args.name.clone())
        .bind(args.age)
        .bind(args.sex.clone())
        .execute(&app_state.pgsql_client.clone().unwrap())
        .await
        .unwrap();

    dbg!("{result}");

    //   let data = util::Data::<PgQueryResult>::new();

    //Json(data.ok(data));
    "ok"
}
