use actix_web::{web, ResponseError, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_identity::Identity;
use bcrypt;

use crate::{Pool, errors::*, models::Account};
use crate::errors::Error;
use jsonwebtoken::{Validation, Algorithm};
use std::time::{UNIX_EPOCH, SystemTime};
use self::r2d2::PooledConnection;
use crate::models::visibility::Visibility;
use crate::models::visibility::Visibility::Private;

static SECRET: &'static str = dotenv!("JWT_SECRET");

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub userid: i64,
    pub email: String,
    pub exp: u64
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub email: String,
    pub password: String,
    pub username: String,
    pub phone: Option<String>
}

pub fn get_identity(ident: Identity) -> ValyouResult<Claims> {
    let jwt = ident.identity().ok_or(Error::Unauthorized)?;

    let validation = Validation::new(Algorithm::HS512);
    jsonwebtoken::decode::<Claims>(&jwt, SECRET.as_ref(), &validation)
        .map_err(|_| Error::Unauthorized)
        .map(|tk| tk.claims)
}

pub async fn show(ident: Identity) -> ValyouResult<HttpResponse> {
    let identity = get_identity(ident)?;
    serde_json::to_string(&identity)
        .map_err(|_| Error::InternalServerError)
        .map(|json| HttpResponse::Ok().body(json))
}

pub async fn register(data: web::Json<CreateRequest>, pool: web::Data<Pool>) -> ValyouResult<HttpResponse> {
    use crate::schema::accounts::dsl::*;

    let db = pool.get()?;

    let dupe_emails: QueryResult<i64> = accounts
        .filter(email.eq(&data.email))
        .count()
        .get_result(&db);
    let dupe_handles: QueryResult<i64> = {
        use crate::schema::usernames::dsl::*;
        usernames
            .filter(handle.eq(&data.username))
            .count()
            .get_result(&db)
    };

    match (dupe_emails, dupe_handles) {
        (Ok(0), Ok(0)) => {
            let hashed_password = bcrypt::hash(&data.password, 9)
                .map_err(|_| Error::InternalServerError)?;

            let userid: i64 = diesel::insert_into(accounts)
                .values((email.eq(&data.email), hash.eq(&hashed_password)))
                .returning(id)
                .get_result(&db)?;

            create_profile(&db, userid)
                .and_then(|_| set_username(&db, userid, &data.username))
                .map_err(|e| Error::from(e))
                .map(|_| HttpResponse::Ok().finish())
        },
        (Ok(n), _) if n > 0 => {
            Err(Error::BadRequest(String::from("Email already in use")))
        },
        (_, Ok(n)) if n > 0 => {
            Err(Error::BadRequest(String::from("Username already in use")))
        },
        _ => {
            Err(Error::InternalServerError)
        }
    }
}

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

fn set_username(db: &Connection, user_id: i64, username: &String) -> QueryResult<usize> {
    use crate::schema::usernames::dsl::*;

    diesel::insert_into(usernames)
        .values((id.eq(user_id), handle.eq(username)))
        .execute(db)
}

fn create_profile(db: &Connection, user_id: i64) -> QueryResult<usize> {
    use crate::schema::profiles::dsl::*;

    diesel::insert_into(profiles)
        .values(&(id.eq(user_id), visibility.eq(Private)))
        .execute(db)
}

pub async fn login(data: web::Json<AuthRequest>, ident: Identity, pool: web::Data<Pool>) -> ValyouResult<HttpResponse> {
    use crate::schema::accounts::dsl::*;

    if ident.identity().is_some() {
        return Err(Error::Unauthorized);
    }

    let account: Account = {
        let db = pool.get()?;

        accounts.filter(email.eq(&data.email))
            .first::<Account>(&db)?
    };

    let verified = bcrypt::verify(&data.password, &account.hash)
        .map_err(|_| Error::InternalServerError)?;

    if verified {
        let header = jsonwebtoken::Header::new(Algorithm::HS512);

        let body = Claims {
            userid: account.id,
            email: account.email,
            exp: get_current_timestamp() + 2419200
        };

        let jwt = jsonwebtoken::encode(&header, &body, SECRET.as_ref())
            .map_err(|_| Error::InternalServerError)?;

        ident.remember(jwt);
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(Error::Unauthorized)
    }
}

fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}

pub async fn logout(id: Identity) -> impl Responder {
    if id.identity().is_none() {
        HttpResponse::Unauthorized().finish()
    } else {
        id.forget();
        HttpResponse::NoContent().finish()
    }
}

pub async fn edit(ident: Identity) -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}