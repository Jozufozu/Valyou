use std::time::{SystemTime, UNIX_EPOCH};

use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, web};
use bcrypt;
use diesel::prelude::*;
use diesel::r2d2;
use dotenv;
use jsonwebtoken::{Algorithm, Validation};

use crate::{errors::*, models::Account, Pool};
use crate::errors::Error;

static SECRET: &'static str = dotenv!("JWT_SECRET");

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub userid: i64,
    pub exp: u64
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub email: String,
    pub password: String,
    pub username: String,
    pub phone: Option<String>
}

pub fn get_identity(ident: &Identity) -> ValyouResult<Claims> {
    let jwt = ident.identity().ok_or(Error::Unauthorized)?;

    let validation = Validation::new(Algorithm::HS512);
    jsonwebtoken::decode::<Claims>(&jwt, SECRET.as_ref(), &validation)
        .map_err(|_| {
            ident.forget();
            Error::Unauthorized
        })
        .map(|tk| {
            set_identity(ident, tk.claims.userid);
            tk.claims
        })
}

pub fn set_identity(ident: &Identity, id: i64) {
    let header = jsonwebtoken::Header::new(Algorithm::HS512);

    let start = SystemTime::now();
    let timestamp = start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

    let body = Claims {
        userid: id,
        exp: timestamp + 2419200
    };

    let jwt = jsonwebtoken::encode(&header, &body, SECRET.as_ref()).unwrap();

    ident.remember(jwt);
}

pub async fn show(ident: Identity) -> RequestResult {
    let identity = get_identity(&ident)?;
    Ok(HttpResponse::Ok().json(identity))
}

pub async fn register(data: web::Json<CreateRequest>, pool: web::Data<Pool>) -> RequestResult {
    use crate::views::new_account::dsl::*;

    let hashed_password = bcrypt::hash(&data.password, 9)
        .map_err(|_| Error::InternalServerError)?;

    diesel::insert_into(new_account)
        .values(&(email.eq(&data.email), hash.eq(&hashed_password), username.eq(&data.username)))
        .execute(&pool.get()?)?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn login(data: web::Json<AuthRequest>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
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
        set_identity(&ident, account.id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(Error::Unauthorized)
    }
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