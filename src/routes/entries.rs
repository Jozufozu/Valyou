use std::cmp::max;

use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use diesel::{prelude::*, QueryDsl};

use crate::errors::RequestResult;
use crate::models::{can_see_entry, entries::Entry};
use crate::models::search::{Paginated, SearchMethod, SearchQuery};
use crate::Pool;
use crate::routes::account::get_identity;
use crate::schema::entries;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub content: String,
    pub significance: Option<f64>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "entries"]
pub struct EditRequest {
    pub content: Option<String>,
    pub significance: Option<f64>,
}

#[derive(Debug, Insertable)]
#[table_name = "entries"]
pub struct NewEntry {
    pub author: i64,
    pub journal: i64,
    pub content: String,
    pub significance: Option<f64>
}

pub async fn create(path: web::Path<i64>, form: web::Json<CreateRequest>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let CreateRequest {
        content,
        significance,
        mut tags
    } = form.into_inner();
    let jid = path.into_inner();

    let claims = get_identity(&ident)?;

    let db = pool.get()?;

    let new_entry = NewEntry {
        author: claims.userid,
        journal: jid,
        content,
        significance
    };

    let new: i64 = {
        use self::entries::dsl::*;
        diesel::insert_into(entries)
            .values(&new_entry)
            .returning(entryid)
            .get_result(&db)?
    };

    if !tags.is_empty() {
        use crate::schema::entry_tags::dsl::*;

        let insert: Vec<_> = tags.iter().map(|t| (entry.eq(new), tag.eq(t))).collect();

        let result = diesel::insert_into(entry_tags)
            .values(&insert)
            .execute(&db);

        if let Err(_) = result {
            tags.clear();
            // TODO: Maybe not silently erase tags?
        }
    }

    find(web::Path::from((jid, new)), ident, pool).await

}

pub async fn edit(path: web::Path<(i64, i64)>, json: web::Json<EditRequest>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let (jid, eid) = path.into_inner();
    let edit = json.into_inner();
    let me = get_identity(&ident)?.userid;

    use crate::schema::entries::dsl::*;

    let entry: Entry = diesel::update(entries)
        .filter(entryid.eq(eid).and(journal.eq(jid)).and(author.eq(me)))
        .set(&edit)
        .returning((entryid, author, journal, created, modified, modifiedc, content, significance))
        .get_result(&pool.get()?)?;

    Ok(HttpResponse::Ok().json(entry))
}

pub async fn in_journal(args: web::Path<(i64, SearchMethod)>, query: web::Query<SearchQuery>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let (journalid, method) = args.into_inner();
    let SearchQuery { id, limit } = query.into_inner();
    let limit = max(limit, 30);

    let me = get_identity(&ident)?.userid;

    let found: Vec<Entry> = {
        use crate::views::visible_entries::dsl::*;

        match method {
            SearchMethod::Before => {
                visible_entries
                    .filter(entryid.lt(id).and(journal.eq(journalid)).and(can_see_entry(me, author, journal)))
                    .order(entryid.desc())
                    .limit(limit)
                    .get_results(&pool.get()?)?
            },
            SearchMethod::After => {
                visible_entries
                    .filter(entryid.gt(id).and(journal.eq(journalid)).and(can_see_entry(me, author, journal)))
                    .order(entryid.asc())
                    .limit(limit)
                    .get_results(&pool.get()?)?
            }
        }
    };

    Ok(HttpResponse::Ok().json(Paginated::paginate(found, method)))
}

pub async fn find(entryid: web::Path<(i64, i64)>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let (jid, eid) = entryid.into_inner();

    let me = get_identity(&ident)?.userid;

    let found: Entry = {
        use crate::views::visible_entries::dsl::*;

        visible_entries
            .filter(entryid.eq(eid).and(journal.eq(jid)).and(can_see_entry(me, author, journal)))
            .get_result(&pool.get()?)?
    };

    Ok(HttpResponse::Ok().json(found))
}
