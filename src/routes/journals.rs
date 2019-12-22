use actix_identity::Identity;
use actix_web::{Responder, HttpResponse, web};
use crate::Pool;
use crate::models::visibility::Visibility;
use crate::errors::{Error, ValyouResult};
use crate::routes::account::get_identity;
use crate::schema::journals;
use diesel::RunQueryDsl;
use crate::models::Journal;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub visibility: Option<Visibility>
}

#[derive(Debug, Insertable)]
#[table_name = "journals"]
pub struct NewJournal {
    pub owner: i64,
    pub name: String,
    pub description: Option<String>,
    pub visibility: Visibility
}

pub async fn create(create: web::Json<CreateRequest>, ident: Identity, pool: web::Data<Pool>) -> ValyouResult<HttpResponse> {
    let identity = get_identity(ident)?;
    let CreateRequest { name, description, visibility } = create.into_inner();

    let new_journal = NewJournal {
        owner: identity.userid,
        name,
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
            .map(|inserted: Journal| HttpResponse::Created().body(serde_json::to_string(&inserted).unwrap()))
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