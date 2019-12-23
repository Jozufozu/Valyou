use actix_identity::Identity;
use actix_web::{Responder, HttpResponse, web};
use diesel::prelude::*;
use crate::errors::{RequestResult, Error};
use crate::routes::account::get_identity;
use crate::models::status::RelationStatus;
use crate::Pool;
use crate::models::Friend;

pub async fn send_request(to: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let friendid = to.into_inner();
    let userid = get_identity(&ident)?.userid;

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

pub async fn accept_request(path: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let friendid = path.into_inner();
    let userid = get_identity(&ident)?.userid;

    if userid == friendid {
        return Err(Error::BadRequest("You're already friends with yourself".into()));
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
        Err(Error::BadRequest(String::new()))
    }
}

pub async fn deny_request(to: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let friendid = to.into_inner();
    let userid = get_identity(&ident)?.userid;

    if userid == friendid {
        return Err(Error::BadRequest("You're always friends with yourself".into()));
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
        return Err(Error::BadRequest("You're always friends with yourself".into()));
    }

    let pair = get_relation_pk(userid, friendid);

    use crate::schema::relations::dsl::*;
    diesel::delete(relations)
        .filter(user_from.eq(pair.0).and(user_to.eq(pair.1)).and(status.eq(RelationStatus::Friends)))
        .execute(&pool.get()?)?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn view_own_friends(ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let userid = get_identity(&ident)?.userid;

    let friends: Vec<Friend> = diesel::sql_query(format!(r#"
SELECT friend_table.friend AS userid, usernames.username, usernames.discriminator, profiles.summary, profiles.bio, friend_table.since
FROM (
    SELECT * FROM (
        SELECT user_to AS friend, status, since
        FROM relations
        WHERE user_from={userid}
        UNION
        SELECT user_from AS friend, status, since
        FROM relations
        WHERE user_to={userid}
    ) AS friend_table
    WHERE friend_table.status='friends'
) AS friend_table
INNER JOIN profiles ON profiles.id=friend_table.friend
INNER JOIN usernames ON usernames.id=friend_table.friend
ORDER BY usernames.handle
"#, userid=userid)).get_results(&pool.get()?)?;

    Ok(HttpResponse::Ok().json(serde_json::to_string(&friends).unwrap()))
}

pub async fn show_requests(ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let userid = get_identity(&ident)?.userid;

    let friends: Vec<Friend> = diesel::sql_query(format!(r#"
SELECT requests.userid, usernames.username, usernames.discriminator, profiles.summary, profiles.bio, requests.since
FROM (
    SELECT user_to AS userid, since
    FROM relations
    WHERE user_from={userid}
    AND status='pending_second_first'
    UNION
    SELECT user_from AS userid, since
    FROM relations
    WHERE user_to={userid}
    AND status='pending_first_second'
) AS requests
INNER JOIN profiles ON profiles.id=requests.userid
INNER JOIN usernames ON usernames.id=requests.userid
ORDER BY requests.since
"#, userid=userid)).get_results(&pool.get()?)?;

    Ok(HttpResponse::Ok().json(serde_json::to_string(&friends).unwrap()))
}

#[inline(always)]
fn get_relation_pk(id1: i64, id2: i64) -> (i64, i64) {
    if id1 < id2 {
        (id1, id2)
    } else {
        (id2, id1)
    }
}