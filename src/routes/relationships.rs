use actix_identity::Identity;
use actix_web::{Responder, HttpResponse, web};
use diesel::prelude::*;
use crate::errors::{RequestResult, Error};
use crate::routes::account::get_identity;
use crate::models::status::RelationStatus;
use crate::Pool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FrqAction {
    Accept,
    Deny
}

pub async fn send_request(to: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let friendid = to.into_inner();
    let userid = get_identity(ident)?.userid;

    if userid == friendid {
        return Err(Error::BadRequest("You're already friends with yourself".into()));
    }

    let (pair, pending) = if userid < friendid {
        ((userid, friendid), RelationStatus::PendingFirstSecond)
    } else {
        ((friendid, userid), RelationStatus::PendingSecondFirst)
    };

    {
        use crate::schema::relations::dsl::*;

        diesel::insert_into(relations)
            .values(&(user_from.eq(pair.0), user_to.eq(pair.1), status.eq(pending)))
            .execute(&pool.get()?);
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn respond_request(path: web::Path<(i64, FrqAction)>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let (friendid, action) = path.into_inner();
    let userid = get_identity(ident)?.userid;

    if userid == friendid {
        return Err(Error::BadRequest("You're already friends with yourself".into()));
    }

    let (pair, required) = if userid < friendid {
        ((userid, friendid), RelationStatus::PendingSecondFirst)
    } else {
        ((friendid, userid), RelationStatus::PendingFirstSecond)
    };

    match action {
        FrqAction::Accept => {
            use crate::schema::relations::dsl::*;
            diesel::update(relations)
                .filter(user_from.eq(pair.0).and(user_to.eq(pair.0)).and(status.eq(required)))
                .set(status.eq(RelationStatus::Friends))
                .execute(&pool.get()?);
        },
        FrqAction::Deny => {
            use crate::schema::relations::dsl::*;
            diesel::delete(relations)
                .filter(user_from.eq(pair.0).and(user_to.eq(pair.1)))
                .execute(&pool.get()?);
        }
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn remove_friend(to: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let friendid = to.into_inner();
    let userid = get_identity(ident)?.userid;

    if userid == friendid {
        return Err(Error::BadRequest("You're always friends with yourself".into()));
    }

    let pair = get_relation_pk(userid, friendid);

    use crate::schema::relations::dsl::*;
    diesel::delete(relations)
        .filter(user_from.eq(pair.0).and(user_to.eq(pair.1)))
        .execute(&pool.get()?)?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_for(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn show_requests(ident: Identity, pool: web::Data<Pool>) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

#[inline(always)]
fn get_relation_pk(id1: i64, id2: i64) -> (i64, i64) {
    if id1 < id2 {
        (id1, id2)
    } else {
        (id2, id1)
    }
}