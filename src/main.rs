#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv_codegen;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate derive_more;
#[macro_use] extern crate actix_rt;
#[macro_use] extern crate actix_web;

use actix_web::{web, middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use std::io;
mod models;
mod schema;
mod errors;
mod routes;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let manager = ConnectionManager::<PgConnection>::new(dotenv!("DATABASE_URL"));
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        use routes::*;
        App::new()
            .data(pool.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("valauth")
                    .secure(false)
            ))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/account")
                    .service(web::scope("/auth")
                        .route("", web::post().to(account::login))
                        .route("", web::delete().to(account::logout))
                    )
                    .route("", web::get().to(account::show))
                    .route("", web::post().to(account::register))
                    .route("", web::patch().to(account::edit))
            )
            .service(web::scope("/user")
                .route("", web::get().to(profiles::search))
                .route("", web::patch().to(profiles::edit))
                .service(web::scope("/{userid}")
                    .service(web::scope("/friends")
                        .route("", web::get().to(relationships::list_for))
                        .route("", web::post().to(relationships::send_request))
                        .route("", web::delete().to(relationships::remove_friend))
                        .route("/request/{method}", web::post().to(relationships::respond_request))
                    )
                    .route("/profile", web::get().to(profiles::view))
                )
                .service(web::scope("/self")
                    .service(
                        web::scope("/friends")
                            .service(web::resource("/request/{method}").to(relationships::show_requests))
                            .route("", web::get().to(profiles::view_self))
                    )
                )
            )
            .service(web::scope("/entry")
                .route("", web::post().to(entries::create))
                .service(web::scope("/{entryid}")
                    .route("", web::get().to(entries::find))
                    .route("", web::patch().to(entries::edit))
                )
            )
            .service(web::scope("/journal")
                .route("", web::get().to(journals::search))
                .route("", web::post().to(journals::create))
                .service(web::scope("/{journalid}")
                    .route("", web::get().to(journals::find))
                    .route("", web::patch().to(journals::edit))
                    .route("/entries/{method}", web::get().to(entries::in_journal))
                )
            )
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .start()
        .await
}