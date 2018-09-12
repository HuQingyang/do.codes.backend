
use mongodb::coll::{Collection};
use actix_web::{HttpRequest};

pub struct AppState {
    pub posts_collection: Collection
}

#[derive(Deserialize, Serialize)]
pub struct RetData<T> {
    pub code: u8,
    pub msg: Option<String>,
    pub data: Option<T>,
}

pub type RequestWithState = HttpRequest<AppState>;
