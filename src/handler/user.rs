use std::collections::HashMap;

use actix_web::{
    web::{self},
    HttpRequest, HttpResponse, Responder, 
};

use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::keys;
use crate::state;
use crate::util;

#[derive(Serialize, Default)]
pub struct User {
    id: i32,
    username: String,
    age: i32,
}

// this function could be located in different module
pub fn config(scfg: &mut web::ServiceConfig) {
    scfg.route("/getAll", web::get().to(get_all))
        .route("/getOne", web::get().to(get_one))
        .route("/add", web::get().to(add))
        .route("/del", web::get().to(del));
}

pub async fn get_all(data: web::Data<state::AppStateData>) -> &'static str {
    let conn = &mut data.redis_client.get_async_connection().await.unwrap();
    let v: HashMap<String, String> = conn.hgetall(keys::users()).await.unwrap();

    println!("zzz = {:?}", v);

    "get_all"
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

pub async fn get_one(
    data: web::Data<state::AppStateData>,
    args: web::Query<GetOneReq>,
) -> impl Responder {
    let conn = &mut data.redis_client.get_async_connection().await.unwrap();

    let key = keys::users();
    let fld = keys::user(args.uid);

    let val: String = conn.hget(key, fld).await.unwrap();

    let data = util::Data::<User>::new();

    HttpResponse::Ok().body(val)
}

pub async fn add(
    data: web::Data<state::AppStateData>,
    args: web::Query<AddOneReq>,
) -> impl Responder {
    let conn = &mut data.redis_client.get_async_connection().await.unwrap();

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
        return web::Json(data.err(1, String::from("haha")));
    };

    web::Json(data.ok(user))
}

pub async fn del(data: web::Data<state::AppStateData>) -> impl Responder {
    let conn = &mut data.redis_client.get_async_connection().await.unwrap();
    let r: i32 = conn.hdel(keys::users(), 1).await.unwrap();

    let data = util::Data::<i32>::new();

    if r == 1 {
        return web::Json(data.ok(r));
    }

    return web::Json(data.err(10, format!("fuck-{}", r)));
}
