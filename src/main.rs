#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use actix_web::{web, middleware, App, HttpResponse, HttpServer, Responder, ResponseError};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_identity::{CookieIdentityPolicy, IdentityService, Identity};
use bcrypt;
use frostflake;
use frostflake::GeneratorPoolOptions;

mod models;
mod schema;
mod auth;
mod errors;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn main() {
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let id_generator = frostflake::GeneratorPool::new(4, frostflake::GeneratorPoolOptions::default());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(id_generator)
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth")
                    .secure(false)
            ))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/account")
                    .route("/auth", web::get().to(auth::login))
                    .route("/auth", web::delete().to(auth::logout))
            )
            .service(
            web::resource("/{id}/{name}").to(index)
            )
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}