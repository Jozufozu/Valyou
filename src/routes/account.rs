use actix_web::{web, middleware, App, ResponseError, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use actix_identity::{CookieIdentityPolicy, IdentityService, Identity};
use bcrypt;

use crate::{Pool, errors::*, models::Account};
use std::str::FromStr;
use crate::errors::ValyouError::{BadRequest, InternalServerError, Unauthorized};
use diesel::expression::dsl::count;
use diesel::sql_types::BigInt;
use jsonwebtoken::{Validation, Algorithm};
use std::error::Error;
use std::time::{UNIX_EPOCH, SystemTime};
use self::r2d2::PooledConnection;
use crate::models::visibility::Publicity;

static SECRET: &'static str = dotenv!("JWT_SECRET");

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggedIn {
    pub userid: i64,
    pub email: String,
    pub exp: u64
}

#[derive(Debug, Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub password: String,
    pub username: String,
    pub phone: Option<String>
}

pub fn get_identity(ident: Identity) -> ValyouResult<LoggedIn> {
    ident.identity()
        .ok_or(ValyouError::Unauthorized)
        .and_then(|jwt| {
            let validation = Validation::new(Algorithm::HS512);
            jsonwebtoken::decode::<LoggedIn>(&jwt, SECRET.as_ref(), &validation)
                .map_err(|e| ValyouError::BadRequest(format!("{:?}", e.into_kind())))
        })
        .map(|tkd| tkd.claims)
}

pub fn verify(ident: Identity, required_user: i64) -> ValyouResult<bool> {
    get_identity(ident)
        .map(|lgi| lgi.userid == required_user)
}

pub async fn show(ident: Identity) -> RequestResult {
    get_identity(ident)
        .map_err(|e| e.error_response())
        .and_then(|tkd| {
            serde_json::to_string(&tkd)
                .map_err(|_| HttpResponse::InternalServerError().finish())
                .map(|json| HttpResponse::Ok().body(json))
        })
}

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub async fn register(data: web::Json<RegisterData>, pool: web::Data<Pool>) -> RequestResult {
    use crate::schema::accounts::dsl::*;

    pool.get()
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .and_then(|db| {
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
                    bcrypt::hash(&data.password, 9)
                        .map_err(|_| HttpResponse::InternalServerError().finish())
                        .and_then(|hashed_password| {
                            diesel::insert_into(accounts)
                                .values((email.eq(&data.email), hash.eq(&hashed_password)))
                                .returning(id)
                                .get_result(&db)
                                .map_err(|_| HttpResponse::Unauthorized().finish())
                        })
                        .and_then(|userid| {
                            create_profile(&db, userid)
                                .and_then(|_| create_visibility(&db, userid))
                                .and_then(|_| set_username(&db, userid, &data.username))
                        })
                },
                (Ok(n), _) if n > 0 => {
                    Err(HttpResponse::BadRequest().body("Email already in use"))
                },
                (_, Ok(n)) if n > 0 => {
                    Err(HttpResponse::BadRequest().body("Username already in use"))
                },
                _ => {
                    Err(HttpResponse::InternalServerError().finish())
                }
            }
        })
}

fn set_username(db: &Connection, user_id: i64, username: &String) -> RequestResult {
    use crate::schema::usernames::dsl::*;

    diesel::insert_into(usernames)
        .values((id.eq(user_id), handle.eq(username)))
        .execute(db)
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| HttpResponse::InternalServerError().body(format!("username: {:?}", e)))
}

fn create_profile(db: &Connection, user_id: i64) -> RequestResult {
    use crate::schema::profiles::dsl::*;

    diesel::insert_into(profiles)
        .values((id.eq(user_id)))
        .execute(db)
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| HttpResponse::InternalServerError().body(format!("profile: {:?}", e)))}

fn create_visibility(db: &Connection, user_id: i64) -> RequestResult {
    use crate::schema::user_visibility::dsl::*;

    diesel::insert_into(user_visibility)
        .values((id.eq(user_id), visibility.eq(Publicity::Private)))
        .execute(db)
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| HttpResponse::InternalServerError().body(format!("visibility: {:?}", e)))}

pub async fn login(data: web::Json<AuthData>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    use crate::schema::accounts::dsl::*;

    if ident.identity().is_some() {
        return Err(HttpResponse::Unauthorized().finish());
    }

    let conn: &PgConnection = &pool.get().unwrap();
    let res: ValyouResult<Vec<Account>> = accounts.filter(email.eq(&data.email))
        .load::<Account>(conn)
        .map_err(|e| ValyouError::from(e));
    res.and_then(|mut items| items.pop().ok_or(ValyouError::Unauthorized))
        .and_then( |account| {
            if let Ok(verified) = bcrypt::verify(&data.password, &account.hash) {
                if verified {
                    let header = jsonwebtoken::Header::new(Algorithm::HS512);

                    let body = LoggedIn {
                        userid: account.id,
                        email: account.email,
                        exp: get_current_timestamp() + 2419200
                    };

                    jsonwebtoken::encode(&header, &body, SECRET.as_ref())
                        .map_err(|_| ValyouError::InternalServerError)
                } else {
                    Err(ValyouError::Unauthorized)
                }
            } else {
                Err(ValyouError::InternalServerError)
            }
        })
        .and_then(|jwt| {
            ident.remember(jwt);
            Ok(HttpResponse::NoContent().finish())
        })
        .map_err(|e| e.error_response())
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