use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use crate::errors::{Error, RequestResult};
use crate::Pool;
use diesel::{prelude::*, QueryDsl};
use crate::schema::{entries, entry_tags};
use crate::models::{Entry, SearchMethod, SearchQuery};
use crate::routes::account::get_identity;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub content: String,
    pub significance: Option<f64>,
    pub tags: Vec<String>,
    pub journal: String,
}

#[derive(Debug, Deserialize)]
pub struct EditRequest {
    pub content: Option<String>,
    pub significance: Option<f64>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "entries"]
pub struct EditEntry {
    pub content: Option<String>,
    pub significance: Option<f64>
}

#[derive(Debug, Insertable)]
#[table_name = "entries"]
pub struct NewEntry {
    pub author: i64,
    pub journal: i64,
    pub content: String,
    pub significance: Option<f64>
}

#[derive(Debug, Serialize)]
pub struct EntryResponse {
    pub id: i64,
    pub author: i64,
    pub journal: i64,
    pub creator: bool,
    pub created: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiedc: Option<chrono::NaiveDateTime>,
    pub content: String,
    pub significance: Option<f64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>
}

impl EntryResponse {
    pub fn new(entry: Entry, tags: Vec<String>, current_user: i64) -> Self {
        EntryResponse {
            id: entry.id,
            author: entry.author,
            journal: entry.journal,
            creator: entry.author == current_user,
            created: entry.created,
            modified: entry.modified,
            modifiedc: entry.modifiedc,
            content: entry.content,
            significance: entry.significance,
            tags
        }
    }
}

pub async fn create(form: web::Json<CreateRequest>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let CreateRequest {
        content,
        significance,
        mut tags,
        journal
    } = form.into_inner();

    let jid = journal.parse::<i64>()
        .map_err(|e| Error::BadRequest(format!("{:?}", e)))?;

    let claims = get_identity(&ident)?;

    let db = pool.get()?;

    let new_entry = NewEntry {
        author: claims.userid,
        journal: jid,
        content,
        significance
    };

    let new: Entry = {
        use self::entries::dsl::*;
        diesel::insert_into(entries)
            .values(&new_entry)
            .returning((entryid, author, journal, created, modified, modifiedc, content, significance))
            .get_result(&db)?
    };

    if !tags.is_empty() {
        use self::entry_tags::dsl::*;

        let insert: Vec<_> = tags.iter().map(|t| (entry.eq(new.id), tag.eq(t))).collect();

        let result = diesel::insert_into(entry_tags)
            .values(&insert)
            .execute(&db);

        if let Err(_) = result {
            tags.clear();
            // TODO: Maybe not silently erase tags?
        }
    }

    let response = EntryResponse::new(new, tags, claims.userid);

    Ok(HttpResponse::Created().json(serde_json::to_string(&response).unwrap()))

}

pub async fn edit(entryid: web::Path<i64>, request: web::Json<EditRequest>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let entryid = entryid.into_inner();
    let EditRequest { content, significance } = request.into_inner();

    let db = pool.get()?;

    Err(Error::NotFound)
}

pub async fn in_journal(args: web::Path<(i64, SearchMethod)>, query: web::Query<SearchQuery>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let (journalid, method) = args.into_inner();
    let SearchQuery { id, limit } = query.into_inner();

    let claims = get_identity(&ident)?;

    let found: Vec<Entry> = {
        use self::entries::dsl::*;

        match method {
            SearchMethod::Before => {
                entries
                    .select((entryid, author, journal, created, modified, modifiedc, content, significance))
                    .filter(entryid.lt(id).and(journal.eq(journalid)))
                    .order(entryid.desc())
                    .limit(limit)
                    .get_results(&pool.get()?)?
            },
            SearchMethod::After => {
                entries
                    .select((entryid, author, journal, created, modified, modifiedc, content, significance))
                    .filter(entryid.gt(id).and(journal.eq(journalid)))
                    .order(entryid.asc())
                    .limit(limit)
                    .get_results(&pool.get()?)?
            },
            _ => todo!("Have to figure out how to do a query like this.")
        }
    };

    let map: Vec<EntryResponse> = found.into_iter().map(|e| EntryResponse::new(e, Vec::with_capacity(0), claims.userid)).collect() ;

    Ok(HttpResponse::Ok().json(serde_json::to_string(&map).unwrap()))
}

pub async fn find(entryid: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let eid = entryid.into_inner();

    let claims = get_identity(&ident)?;

    let db = pool.get()?;

    let found: Entry = {
        use self::entries::dsl::*;

        entries
            .select((entryid, author, journal, created, modified, modifiedc, content, significance))
            .filter(entryid.eq(eid))
            .get_result(&db)?
    };

    let tags: Vec<String> = {
        use self::entry_tags::dsl::*;

        entry_tags
            .filter(entry.eq(&found.id))
            .select(tag)
            .get_results(&db)?
    };

    let response = EntryResponse::new(found, tags, claims.userid);

    Ok(HttpResponse::Ok().json(serde_json::to_string(&response).unwrap()))
}
