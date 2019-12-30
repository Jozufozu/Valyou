use std::cmp::max;

use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use diesel::prelude::*;

use crate::errors::RequestResult;
use crate::models::can_see_entry;
use crate::models::entries::Entry;
use crate::models::search::{Paginated, SearchMethod, SearchQuery};
use crate::Pool;
use crate::routes::account::get_identity;

pub async fn timeline(path: web::Path<SearchMethod>, query: web::Query<SearchQuery>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let method = path.into_inner();
    let SearchQuery { id, limit } = query.into_inner();

    let limit = max(limit, 30);

    let me = get_identity(&ident)?.userid;

    let found: Vec<Entry> = {
        use crate::views::visible_entries::dsl::*;

        match method {
            SearchMethod::Before => {
                visible_entries
                    .filter(entryid.lt(id).and(author.eq(me)))
                    .order(entryid.desc())
                    .limit(limit)
                    .get_results(&pool.get()?)?
            },
            SearchMethod::After => {
                visible_entries
                    .filter(entryid.gt(id).and(author.eq(me)))
                    .order(entryid.asc())
                    .limit(limit)
                    .get_results(&pool.get()?)?
            }
        }
    };

    Ok(HttpResponse::Ok().json(Paginated::paginate(found, method)))
}

pub async fn feed(args: web::Path<SearchMethod>, query: web::Query<SearchQuery>, ident: Identity, pool: web::Data<Pool>) -> RequestResult {
    let method = args.into_inner();
    let SearchQuery { id, limit } = query.into_inner();
    let limit = max(limit, 30);

    let me = get_identity(&ident)?.userid;

    let found: Vec<Entry> = {
        use crate::views::visible_entries::dsl::*;

        match method {
            SearchMethod::Before => {
                visible_entries
                    .filter(entryid.lt(id).and(can_see_entry(me, author, journal)))
                    .order(entryid.desc())
                    .limit(limit)
                    .get_results(&pool.get()?)?
            },
            SearchMethod::After => {
                visible_entries
                    .filter(entryid.gt(id).and(can_see_entry(me, author, journal)))
                    .order(entryid.asc())
                    .limit(limit)
                    .get_results(&pool.get()?)?
            }
        }
    };

    Ok(HttpResponse::Ok().json(Paginated::paginate(found, method)))
}