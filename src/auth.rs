use actix_web::{web, middleware, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_identity::{CookieIdentityPolicy, IdentityService, Identity};
use bcrypt;
use frostflake;

use crate::{Pool, errors::*, models::Account};
use std::str::FromStr;
use crate::errors::ServiceError::BadRequest;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String
}

pub fn verify(id: Identity, db: web::Data<Pool>) -> Result<u64> {
    if let Some(userid) = id.identity() {
        u64::from_str(&userid).map_err(|| BadRequest("Invalid auth cookie".to_owned()))
    } else {
        Err(ServiceError::Unauthorized)
    }
}

pub async fn login(data: web::Json<AuthData>, id: Identity, db: web::Data<Pool>) -> impl Responder {
    use crate::schema::accounts::dsl::*;

    let conn: &PgConnection = &pool.get().unwrap();
    let mut items = accounts.filter(email.eq(&data.email)).load::<Account>(conn)?;

    if let Some(account) = items.pop() {
        if bcrypt::verify(&data.password, &account.hash) {
            id.remember(String::from(&account.id));
        }
    }

    HttpResponse::BadRequest().finish()
}

pub async fn logout(id: Identity) -> impl Responder {
    if id.identity().is_none() {
        HttpResponse::Unauthorized().finish()
    } else {
        id.forget();
        HttpResponse::NoContent().finish()
    }
}