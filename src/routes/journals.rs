use std::cmp::min;

use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

use crate::errors::RequestResult;
use crate::models::can_see;
use crate::models::Journal;
use crate::models::search::{Paginated, SearchMethod, SearchQuery};
use crate::models::visibility::Visibility;
use crate::Pool;
use crate::routes::account::get_identity;
use crate::schema::journals;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub title: String,
    pub description: Option<String>,
    pub visibility: Option<Visibility>
}

#[derive(Debug, Insertable)]
#[table_name = "journals"]
pub struct NewJournal {
    pub owner: i64,
    pub title: String,
    pub description: Option<String>,
    pub visibility: Visibility
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "journals"]
pub struct EditRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<Visibility>
}

pub async fn create(create: web::Json<CreateRequest>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let identity = get_identity(&ident)?;
    let CreateRequest { title, description, visibility } = create.into_inner();

    let new_journal = NewJournal {
        owner: identity.userid,
        title,
        description,
        visibility: visibility.unwrap_or(Visibility::Private)
    };

    let db = pool.get()?;

    let journal: Journal = {
        use self::journals::dsl::*;

        diesel::insert_into(journals)
            .values(&new_journal)
            .get_result(&db)?
    };

    Ok(HttpResponse::Created().json(journal))
}

pub async fn edit(path: web::Path<i64>, json: web::Json<EditRequest>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let jid = path.into_inner();
    let me = get_identity(&ident)?.userid;

    use self::journals::dsl::*;
    let journal: Journal = diesel::update(journals)
        .filter(journalid.eq(jid).and(owner.eq(me)))
        .set(json.into_inner())
        .get_result(&pool.get()?)?;

    Ok(HttpResponse::Ok().json(journal))
}

pub async fn find(path: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let jid = path.into_inner();

    let me = get_identity(&ident)?.userid;

    use self::journals::dsl::*;
    let found: Journal = journals
            .filter(journalid.eq(jid).and(can_see(me, owner, journalid)))
            .get_result(&pool.get()?)?;

    Ok(HttpResponse::Ok().json(found))
}

pub async fn get_own_journals(path: web::Path<SearchMethod>, query: web::Query<SearchQuery>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    let method = path.into_inner();
    let (id, limit) = query.into_inner().into_parts();

    use self::journals::dsl::*;
    let found: Vec<Journal> = match method {
        SearchMethod::Before => {
            journals
                .filter(journalid.lt(id).and(owner.eq(me)))
                .order(journalid.desc())
                .limit(limit)
                .get_results(&pool.get()?)?
        },
        SearchMethod::After => {
            journals
                .filter(journalid.gt(id).and(owner.eq(me)))
                .order(journalid.asc())
                .limit(limit)
                .get_results(&pool.get()?)?
        }
    };

    Ok(HttpResponse::Ok().json(Paginated::paginate(found, method)))
}

pub async fn get_user_journals(path: web::Path<(i64, SearchMethod)>, query: web::Query<SearchQuery>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let me = get_identity(&ident)?.userid;

    let (user, method) = path.into_inner();
    let (id, limit) = query.into_inner().into_parts();

    use self::journals::dsl::*;
    let found: Vec<Journal> = match method {
        SearchMethod::Before => {
            journals
                .filter(journalid.lt(id).and(owner.eq(user)).and(can_see(me, user, journalid)))
                .order(journalid.desc())
                .limit(limit)
                .get_results(&pool.get()?)?
        },
        SearchMethod::After => {
            journals
                .filter(journalid.gt(id).and(owner.eq(user)).and(can_see(me, user, journalid)))
                .order(journalid.asc())
                .limit(limit)
                .get_results(&pool.get()?)?
        }
    };

    Ok(HttpResponse::Ok().json(Paginated::paginate(found, method)))
}

pub async fn search(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}