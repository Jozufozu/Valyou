#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv_codegen;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate derive_more;
#[macro_use] extern crate actix_rt;
#[macro_use] extern crate actix_web;

use actix_web::{web, middleware, App, HttpServer, Responder, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use std::io;
use env_logger;

mod models;
mod schema;
mod schemas;
mod errors;
mod routes;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn missing() -> impl Responder {
    HttpResponse::NotFound().finish()
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

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
            .default_service(web::to(missing))
            .service(
                web::scope("/account")
                    .route("", web::get().to(account::show))
                    .route("", web::post().to(account::register))
                    .route("", web::patch().to(account::edit))
                    .service(web::scope("/auth")
                        .route("", web::post().to(account::login))
                        .route("", web::delete().to(account::logout))
                    )
            )
            .service(web::scope("/user")
                .route("", web::get().to(profiles::search))
                .service(web::scope("/self")
                    .route("/friends", web::get().to(relationships::view_own_friends))
                    .route("/friends/request", web::get().to(relationships::show_requests))
                    .service(web::scope("/profile")
                        .route("", web::get().to(profiles::view_self))
                        .route("", web::patch().to(profiles::edit))
                        .route("/set_visibility/{visibility}", web::patch().to(profiles::set_visibility))
                    )
                )
                .service(web::scope("/{userid}")
                    .route("/profile", web::get().to(profiles::view))
                    .service(web::scope("/friends")
                        //.route("", web::get().to(relationships::list_for))
                        .route("", web::post().to(relationships::send_request))
                        .route("", web::delete().to(relationships::remove_friend))
                        .service(web::scope("/request")
                            .route("", web::post().to(relationships::accept_request))
                            .route("", web::delete().to(relationships::deny_request))
                        )
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
                    .route("/entries", web::get().to(entries::in_journal))
                )
            )
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .start()
        .await
}