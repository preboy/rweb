use std::collections::HashMap;
use std::sync::Arc;

use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json;

use axum::{
    extract::{Json, Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::util;
use crate::{keys, state::AppState};

#[derive(Serialize, Default)]
pub struct User {
    id: i32,
    username: String,
    age: i32,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/getAll", get(get_all))
        .route("/getOne", get(get_one))
        .route("/add", get(add))
        .route("/del", get(del))
}

#[derive(Deserialize)]
pub struct GetOneReq {
    uid: i32,
}

#[derive(Deserialize)]
pub struct AddOneReq {
    username: String,
    age: i32,
}

async fn get_all(State(app_state): State<Arc<AppState>>) -> &'static str {
    let conn = &mut app_state.redis_client.get_async_connection().await.unwrap();
    let v: HashMap<String, String> = conn.hgetall(keys::users()).await.unwrap();

    println!("zzz = {:?}", v);

    "get_all"
}

async fn get_one(
    State(app_state): State<Arc<AppState>>,
    args: Query<GetOneReq>,
) -> impl IntoResponse {
    let conn = &mut app_state.redis_client.get_async_connection().await.unwrap();

    let key = keys::users();
    let fld = keys::user(args.uid);

    let _val: String = conn.hget(key, fld).await.unwrap();

    log::info!("haha{}", _val);
    let data = util::Data::<User>::new();

    Json(data)
}

async fn add(State(app_state): State<Arc<AppState>>, args: Query<AddOneReq>) -> impl IntoResponse {
    let conn = &mut app_state.redis_client.get_async_connection().await.unwrap();

    let uid = 101_i32;

    let user = User {
        id: uid,
        age: args.age,
        username: args.username.clone(),
    };

    let str = serde_json::to_string(&user).unwrap();

    let rv: i32 = conn
        .hset(keys::users(), keys::user(uid), str.clone())
        .await
        .unwrap();

    println!("rv = {:?}   {}", rv, str);

    let n = 1;
    let data = util::Data::<User>::new();

    if n > 0 {
        return Json(data.err(1, String::from("haha")));
    };

    Json(data.ok(user))
}

async fn del(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let conn = &mut app_state.redis_client.get_async_connection().await.unwrap();
    let r: i32 = conn.hdel(keys::users(), 1).await.unwrap();

    let data = util::Data::<i32>::new();

    log::debug!("fuckyou   1");

    if r == 1 {
        return Json(data.ok(r));
    }

    log::debug!("fuckyou   2");

    Json(data.err(10, format!("fuck-{}", r)))
}
