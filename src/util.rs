use serde::Serialize;

#[derive(Serialize)]
pub struct Data<T>
where
    T: Serialize,
{
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T> Data<T>
where
    T: Serialize + Default,
{
    pub fn new() -> Self {
        Data {
            code: 1,
            message: String::from("error"),
            data: None,
        }
    }

    pub fn ok(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn err(mut self, err: i32, msg: String) -> Self {
        self.code = err;
        self.message = msg;
        self
    }
}
