use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use sqlx::{mysql::MySqlRow, Row};

use crate::state::AppState;
use crate::util;

pub fn router() -> Router<Arc<AppState>> {
    // <Arc<AppState>>
    Router::new().route("/", get(root))
}

async fn root(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let rows: Vec<MySqlRow> = sqlx::query("SELECT * FROM `t1`;")
        .fetch_all(&app_state.mysql_client)
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
