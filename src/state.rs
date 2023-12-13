use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: Client,
    pub counter: usize,
}

pub fn new() -> AppState {
    AppState {
        redis_client: redis::Client::open("redis://127.0.0.1/").unwrap(),
        counter: 0,
    }
}
