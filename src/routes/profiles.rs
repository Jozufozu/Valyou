use std::cmp::min;

use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use diesel::prelude::*;

use crate::errors::{RequestResult, ValyouResult};
use crate::models::{self, can_see, can_see_user, Journal};
use crate::models::profiles::{FullProfile, Profile};
use crate::models::search::{Paginated, SearchMethod};
use crate::models::visibility::Visibility;
use crate::Pool;
use crate::routes::account::get_identity;
use crate::schema::profiles;

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "profiles"]
pub struct EditRequest {
    pub summary: Option<String>,
    pub bio: Option<String>,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Deserialize)]
pub struct ChangeUsername {
    pub username: String
}

#[derive(Debug, Deserialize)]
pub struct Search {
    pub q: String,
    pub count: i64
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub profile: FullProfile,
    pub journals: Paginated<Journal>,
}

pub async fn edit(request: web::Json<EditRequest>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    use crate::schema::profiles::dsl::*;
    diesel::update(profiles)
        .set(request.into_inner())
        .filter(userid.eq(me))
        .execute(&pool.get()?)?;

    Ok(HttpResponse::Ok().json(get_profile(me, &pool)?))
}

pub async fn change_username(request: web::Json<ChangeUsername>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    use crate::schema::usernames::dsl::*;
    diesel::update(usernames)
        .set(username.eq(&request.username))
        .filter(userid.eq(me))
        .execute(&pool.get()?)?;

    Ok(HttpResponse::Ok().json(get_profile(me, &pool)?))
}

pub async fn view_self(ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    let profile = get_profile(me, &pool)?;

    let journals: Paginated<Journal> = {
        use crate::schema::journals::dsl::*;
        let out: Vec<Journal> = journals
            .filter(owner.eq(me))
            .limit(10)
            .get_results(&pool.get()?)?;

        Paginated::paginate(out, SearchMethod::Before)
    };

    Ok(HttpResponse::Ok().json(ProfileResponse { profile, journals }))
}

fn get_profile(user: i64, pool: &web::Data<Pool>) -> ValyouResult<FullProfile> {
    use crate::views::full_profiles::dsl::*;
    let profile: FullProfile = full_profiles
        .filter(userid.eq(user))
        .get_result(&pool.get()?)?;

    Ok(profile)
}

pub async fn search(query: web::Query<Search>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    let Search { q, count } = query.into_inner();

    let q = format!("%{}%", q.replace('%', "\\%"));

    let count = min(count, 30);

    use crate::views::searchable::dsl::*;
    let results: Vec<Profile> = searchable
        .filter(userid.ne(me).and(username.like(q).escape('\\')))
        .limit(count)
        .get_results(&pool.get()?)?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn view(path: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;
    let person = path.into_inner();

    let profile: FullProfile = {
        use crate::views::full_profiles::dsl::*;
        full_profiles
            .filter(userid.eq(person).and(can_see_user(me, person)))
            .get_result(&pool.get()?)?
    };

    let journals: Paginated<Journal> = {
        use crate::schema::journals::dsl::*;
        let out: Vec<Journal> = journals
            .filter(owner.eq(person).and(can_see(me, person, journalid)))
            .limit(10)
            .get_results(&pool.get()?)?;

        Paginated::paginate(out, SearchMethod::Before)
    };

    Ok(HttpResponse::Ok().json(ProfileResponse { profile, journals }))
}
