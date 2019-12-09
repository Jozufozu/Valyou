use actix_web::{web, middleware, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_identity::{CookieIdentityPolicy, IdentityService, Identity};
use bcrypt;
use frostflake;

use crate::{Pool, errors::*, models::Account};
use std::str::FromStr;
use crate::errors::Error::BadRequest;
use frostflake::GeneratorPool;
use diesel::expression::dsl::count;
use crate::models::CreateAccount;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
}

pub async fn show(ident: Identity) -> impl Responder {
    if let Some(name) = ident.identity() {
        name
    } else {
        "Not logged in".to_owned()
    }
}

pub async fn register(data: web::Json<RegisterData>, db: web::Data<Pool>) -> impl Responder {
    use crate::schema::accounts::dsl::*;

    if let Ok(hashed_password) = bcrypt::hash(&data.password, 9) {
        let new_account = CreateAccount {
            email: data.email.clone(),
            hash: hashed_password,
            phone: data.phone.clone()
        };
        diesel::insert_into(accounts)
            .values(new_account)
            .execute(&db.get().unwrap());

        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
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