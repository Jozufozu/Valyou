use actix_identity::Identity;
use actix_web::{Responder, HttpResponse};

pub struct Journal {

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

pub async fn search(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}