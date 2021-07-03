use crate::auth::ports::*;
use rocket::{post, routes, serde::json::Json, Route, State};

pub fn configure() -> Vec<Route> {
    routes![register, login, authenticate]
}

#[post("/register", data = "<body>")]
pub async fn register(service: &State<Box<dyn AuthService>>, body: Json<Credential>) -> Json<bool> {
    Json(service.register(&body).await)
}

#[post("/login", data = "<body>")]
async fn login(
    service: &State<Box<dyn AuthService>>,
    body: Json<Credential>,
) -> Json<Option<String>> {
    Json(service.login(&body).await)
}

#[post("/authenticate", data = "<body>")]
async fn authenticate(
    service: &State<Box<dyn AuthService>>,
    body: Json<String>,
) -> Json<Option<String>> {
    Json(service.authenticate(&body).await)
}
