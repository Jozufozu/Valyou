use actix_identity::Identity;
use actix_web::{Responder, HttpResponse};

pub async fn send_request(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn remove_friend(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn list_for(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn show_requests(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}