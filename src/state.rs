use std::sync::Mutex;

use redis::Client;

pub struct AppStateData {
    pub redis_client: Client,
    pub counter: Mutex<usize>,
}

pub fn new() -> AppStateData {
    AppStateData {
        redis_client: redis::Client::open("redis://127.0.0.1/").unwrap(),
        counter: Mutex::new(0),
    }
}
