#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub fn get_users() -> Vec<User> {
    vec![
        User {
            username: "exodus".into(),
            password: "exodus123".into(),
        }, 
        

    ]
}

use tide::{Request, Response, StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

use crate::state::AppState;

pub async fn login(mut req: Request<AppState>) -> tide::Result {
    let payload: LoginPayload = req.body_json().await?;
    let users = get_users();

    let user_valid = users.iter().any(|u| {
        u.username == payload.username && u.password == payload.password
    });

    if user_valid {
        Ok(Response::new(StatusCode::Ok))
    } else {
        Ok(Response::new(StatusCode::Unauthorized))
    }
}
