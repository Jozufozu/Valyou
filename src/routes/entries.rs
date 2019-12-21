use actix_identity::Identity;
use actix_web::{Responder, HttpResponse, web};
use crate::models::visibility::Visibility;
use crate::errors::{RequestResult, Error, ValyouResult};
use crate::Pool;
use diesel::{prelude::*, QueryDsl};
use crate::schema::{entries, entry_tags};
use crate::models::{Entry, SearchMethod, SearchQuery};
use crate::routes::account::get_identity;


macro_rules! entries_and_friends {
    ($user:expr) => {
        {
            use crate::schema::entries::dsl::*;
            use crate::schema::relations::dsl::*;

            entries
                .left_join(relations.on(
                    user_from.eq(author).and(user_to.eq($user))
                        .or(user_to.eq(author).and(user_from.eq($user)))
                ))
        }
    };
}

macro_rules! visible_post {
    ($user:expr) => {
        {
            use crate::schema::entries::dsl::*;
            use crate::schema::relations::dsl::*;


            author.eq($user)
                .or(visibility.eq(Visibility::Public))
                .or(
                    visibility.eq(Visibility::Friends)
                        .and(
                            user_from.eq(author).and(user_to.eq($user))
                                .or(user_to.eq(author).and(user_from.eq($user)))
                        )
                )
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub content: String,
    pub significance: Option<f64>,
    pub tags: Vec<String>,
    pub journal: String,
    pub visibility: Visibility
}

#[derive(Debug, Insertable)]
#[table_name = "entries"]
pub struct NewEntry {
    pub author: i64,
    pub journal: i64,
    pub visibility: Visibility,
    pub content: String,
    pub significance: Option<f64>
}

#[derive(Debug, Serialize)]
pub struct EntryResponse {
    pub id: i64,
    pub author: i64,
    pub journal: i64,
    pub creator: bool,
    pub visibility: Visibility,
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
            visibility: entry.visibility,
            created: entry.created,
            modified: entry.modified,
            modifiedc: entry.modifiedc,
            content: entry.content,
            significance: entry.significance,
            tags
        }
    }
}

pub async fn create(form: web::Json<CreateRequest>, ident: Identity, pool: web::Data<Pool>) -> ValyouResult<HttpResponse> {
    let CreateRequest {
        content,
        significance,
        tags,
        journal,
        visibility
    } = form.into_inner();

    let jid = journal.parse::<i64>()
        .map_err(|e| Error::BadRequest(format!("{:?}", e)))?;

    let claims = get_identity(ident)?;

    let db = pool.get()?;

    let journal_exists: bool = {
        use crate::schema::journals::dsl::*;

        let count: i64 = journals
            .find(jid)
            .filter(owner.eq(claims.userid))
            .count()
            .get_result(&db)
            .map_err(|_| Error::InternalServerError)?;

        count > 0
    };

    if journal_exists {
        let new_entry = NewEntry {
            author: claims.userid,
            journal: jid,
            visibility,
            content,
            significance
        };

        let new: Entry = {
            use self::entries::dsl::*;
            diesel::insert_into(entries)
                .values(&new_entry)
                .returning((id, author, journal, visibility, created, modified, modifiedc, content, significance))
                .get_result(&db)?
        };

        if !tags.is_empty() {
            use self::entry_tags::dsl::*;

            let insert: Vec<_> = tags.iter().map(|t| (entry.eq(new.id), tag.eq(t))).collect();

            diesel::insert_into(entry_tags)
                .values(&insert)
                .execute(&db)?;
        }

        let response = EntryResponse::new(new, tags, claims.userid);

        Ok(HttpResponse::Created().body(serde_json::to_string(&response).unwrap()))
    } else {
        Err(Error::BadRequest(String::from("Journal does not exist")))
    }

}

pub async fn edit(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn in_journal(args: web::Path<(i64, SearchMethod)>, query: web::Query<SearchQuery>, ident: Identity, pool: web::Data<Pool>) -> ValyouResult<HttpResponse> {
    let (journal, method) = args.into_inner();
    let SearchQuery { id, limit } = query.into_inner();
    let searchid = id;

    let claims = get_identity(ident)?;

    let db = pool.get()?;

    let found: Vec<Entry> = {
        use self::entries::dsl::*;

        match method {
            SearchMethod::Before => {
                entries_and_friends!(claims.userid)
                    .filter(id.lt(searchid).and(visible_post!(claims.userid)))
                    .order(id.desc())
                    .select((id, author, journal, visibility, created, modified, modifiedc, content, significance))
                    .limit(limit)
                    .get_results(&db)?
            },
            SearchMethod::After => {
                entries_and_friends!(claims.userid)
                    .filter(id.gt(searchid).and(visible_post!(claims.userid)))
                    .select((id, author, journal, visibility, created, modified, modifiedc, content, significance))
                    .order(id.asc())
                    .limit(limit)
                    .get_results(&db)?
            },
            _ => todo!("Have to figure out how to do a query like this.")
        }
    };

    let map: Vec<EntryResponse> = found.into_iter().map(|e| EntryResponse::new(e, Vec::with_capacity(0), claims.userid)).collect() ;

    Ok(HttpResponse::Ok().body(serde_json::to_string(&map).unwrap()))
}

pub async fn find(entryid: web::Path<i64>, ident: Identity, pool: web::Data<Pool>) -> ValyouResult<HttpResponse> {
    let entryid = entryid.into_inner();

    let claims = get_identity(ident)?;

    let db = pool.get()?;

    let found: Entry = {
        use self::entries::dsl::*;

        entries_and_friends!(claims.userid)
            .filter(id.eq(entryid).and(visible_post!(claims.userid)))
            .select((id, author, journal, visibility, created, modified, modifiedc, content, significance))
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

    Ok(HttpResponse::Ok().body(serde_json::to_string(&response).unwrap()))
}
