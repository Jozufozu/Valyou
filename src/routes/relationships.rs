use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use diesel::prelude::*;

use crate::errors::{Error, RequestResult};
use crate::models::profiles::Friend;
use crate::models::search::{Paginated, SearchMethod};
use crate::models::status::RelationStatus;
use crate::Pool;
use crate::routes::account::get_identity;

pub async fn send_request(to: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let friendid = to.into_inner();
    let userid = get_identity(&ident)?.userid;

    if userid == friendid {
        return Err(Error::BadRequest("provided own userid".into()));
    }

    let (pair, pending) = if userid < friendid {
        ((userid, friendid), RelationStatus::PendingFirstSecond)
    } else {
        ((friendid, userid), RelationStatus::PendingSecondFirst)
    };

    use crate::schema::relations::dsl::*;

    diesel::insert_into(relations)
        .values(&(user_from.eq(pair.0), user_to.eq(pair.1), status.eq(pending)))
        .execute(&pool.get()?)?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn accept_request(path: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let friendid = path.into_inner();
    let userid = get_identity(&ident)?.userid;

    if userid == friendid {
        return Err(Error::BadRequest("provided own userid".into()));
    }

    let (pair, required) = if userid < friendid {
        ((userid, friendid), RelationStatus::PendingSecondFirst)
    } else {
        ((friendid, userid), RelationStatus::PendingFirstSecond)
    };

    let success: usize = {
        use crate::schema::relations::dsl::*;
        diesel::update(relations)
            .filter(user_from.eq(pair.0).and(user_to.eq(pair.1)).and(status.eq(required)))
            .set(status.eq(RelationStatus::Friends))
            .execute(&pool.get()?)?
    };

    if success > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(Error::BadRequest("no request from that user".into()))
    }
}

pub async fn deny_request(to: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let friendid = to.into_inner();
    let userid = get_identity(&ident)?.userid;

    if userid == friendid {
        return Err(Error::BadRequest("provided own userid".into()));
    }

    let (pair, required) = if userid < friendid {
        ((userid, friendid), RelationStatus::PendingSecondFirst)
    } else {
        ((friendid, userid), RelationStatus::PendingFirstSecond)
    };

    use crate::schema::relations::dsl::*;
    diesel::delete(relations)
        .filter(user_from.eq(pair.0).and(user_to.eq(pair.1)).and(status.eq(required)))
        .execute(&pool.get()?)?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn remove_friend(to: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let friendid = to.into_inner();
    let userid = get_identity(&ident)?.userid;

    if userid == friendid {
        return Err(Error::BadRequest("provided own userid".into()));
    }

    let pair = get_relation_pk(userid, friendid);

    use crate::schema::relations::dsl::*;
    diesel::delete(relations)
        .filter(user_from.eq(pair.0).and(user_to.eq(pair.1)).and(status.eq(RelationStatus::Friends)))
        .execute(&pool.get()?)?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn view_own_friends(ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    let friends: Vec<Friend> = {
        use crate::views::public_friends::dsl::*;

        public_friends
            .select((friend, username, discriminator, summary, bio, since))
            .filter(userid.eq(me))
            .order(friend.asc())
            .get_results(&pool.get()?)?
    };

    Ok(HttpResponse::Ok().json(Paginated::paginate(friends, SearchMethod::After)))
}

pub async fn show_requests(ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    let friends: Vec<Friend> = {
        use crate::views::friend_requests::dsl::*;

        friend_requests
            .select((friend, username, discriminator, summary, bio, since))
            .filter(userid.eq(me))
            .order(friend.asc())
            .get_results(&pool.get()?)?
    };

    Ok(HttpResponse::Ok().json(Paginated::paginate(friends, SearchMethod::After)))
}

#[inline(always)]
fn get_relation_pk(id1: i64, id2: i64) -> (i64, i64) {
    if id1 < id2 {
        (id1, id2)
    } else {
        (id2, id1)
    }
}