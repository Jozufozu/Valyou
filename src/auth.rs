use actix_web::{web, middleware, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_identity::{CookieIdentityPolicy, IdentityService, Identity};
use bcrypt;

use crate::{Pool, errors::*, models::Account};
use std::str::FromStr;
use crate::errors::Error::BadRequest;
use diesel::expression::dsl::count;
use diesel::sql_types::BigInt;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub password: String,
    pub username: String,
}

pub async fn show(ident: Identity) -> impl Responder {
    if let Some(name) = ident.identity() {
        name
    } else {
        "Not logged in".to_owned()
    }
}

pub async fn register(data: web::Json<RegisterData>, db: web::Data<Pool>) -> RequestResult {
    use crate::schema::accounts::dsl::*;

    bcrypt::hash(&data.password, 9)
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .and_then(|hashed_password| {
            diesel::insert_into(accounts)
                .values((email.eq(&data.email), hash.eq(&hashed_password)))
                .returning(id)
                .get_result(&db.get().unwrap())
                .map_err(|_| HttpResponse::Unauthorized().finish())
                .and_then(|user_id| set_username(db, user_id, &data.username))
        })
}

fn set_username(db: web::Data<Pool>, user_id: i64, username: &String) -> RequestResult {
    use crate::schema::usernames::dsl::*;

    diesel::insert_into(usernames)
        .values((id.eq(user_id), handle.eq(username)))
        .execute(&db.get().unwrap())
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|_| HttpResponse::Unauthorized().finish())
}

pub async fn login(data: web::Json<AuthData>, ident: Identity, pool: web::Data<Pool>) -> impl Responder {
    use crate::schema::accounts::dsl::*;

    if ident.identity().is_some() {
        return HttpResponse::Unauthorized().finish();
    }

    let conn: &PgConnection = &pool.get().unwrap();
    if let Ok(mut items) = accounts.filter(email.eq(&data.email)).load::<Account>(conn) {
        if let Some(account) = items.pop() {
            if let Ok(verified) = bcrypt::verify(&data.password, &account.hash) {
                if verified {
                    ident.remember((&account).id.to_string());

                    return HttpResponse::Ok().body(serde_json::to_string(&account).unwrap());
                }
            }
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