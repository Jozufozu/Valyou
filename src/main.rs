#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate derive_more;

use actix_web::{web, middleware, App, HttpResponse, HttpServer, Responder, ResponseError};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_identity::{CookieIdentityPolicy, IdentityService, Identity};
use bcrypt;

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

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth")
                    .secure(false)
            ))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/account")
                    .service(web::scope("/auth")
                        .route("", web::post().to(auth::login))
                        .route("", web::delete().to(auth::logout))
                    )
                    .route("", web::post().to(auth::register))
                    .route("", web::get().to(auth::show))
            )
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}