use actix_identity::Identity;
use actix_web::{Responder, HttpResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub id: i64,
    pub username: String,
    pub summary: Option<String>,
    pub bio: Option<String>,
    pub friends: bool
}

pub async fn edit(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn search(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn view_self(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn view(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}
