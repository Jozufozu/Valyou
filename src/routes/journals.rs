use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, web};
use diesel::RunQueryDsl;

use crate::errors::{Error, ValyouResult};
use crate::models::Journal;
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

pub async fn create(create: web::Json<CreateRequest>, ident: Identity, pool: web::Data<Pool>) -> ValyouResult<HttpResponse> {
    let identity = get_identity(&ident)?;
    let CreateRequest { title, description, visibility } = create.into_inner();

    let new_journal = NewJournal {
        owner: identity.userid,
        title,
        description,
        visibility: visibility.unwrap_or(Visibility::Private)
    };

    let db = pool.get()?;

    {
        use self::journals::dsl::*;

        diesel::insert_into(journals)
            .values(&new_journal)
            .get_result(&db)
            .map_err(|e| Error::from(e))
            .map(|inserted: Journal| HttpResponse::Created().json(inserted))
    }
}

pub async fn edit(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn find(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub async fn search(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}