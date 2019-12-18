use actix_identity::Identity;
use actix_web::{Responder, HttpResponse};

pub struct Entry {

}

pub async fn create(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn edit(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn find(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}