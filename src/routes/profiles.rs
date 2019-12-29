use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

use crate::errors::RequestResult;
use crate::models::visibility::Visibility;
use crate::Pool;
use crate::routes::account::get_identity;
use crate::models::profiles::OwnProfile;

#[derive(Debug, Deserialize)]
pub struct EditRequest {
    pub username: Option<String>,
    pub summary: Option<String>,
    pub bio: Option<String>,
}

pub async fn edit(request: web::Json<EditRequest>, ident: Identity, pool: web::Data<Pool>) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn set_visibility(to: web::Path<Visibility>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    use crate::schema::profiles::dsl::*;
    diesel::update(profiles)
        .set(visibility.eq(to.into_inner()))
        .filter(userid.eq(me))
        .execute(&pool.get()?)?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn view_self(ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    let me: OwnProfile = {
        use crate::schema::usernames::dsl::{usernames, username, discriminator, modified as umodified};
        use crate::schema::profiles::dsl::*;

        profiles.inner_join(usernames)
            .filter(userid.eq(me))
            .select((userid, username, discriminator, summary, bio, visibility, modified, umodified))
            .get_result(&pool.get()?)?
    };

    Ok(HttpResponse::Ok().json(me))
}

pub async fn search(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()

}

pub async fn view(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}
