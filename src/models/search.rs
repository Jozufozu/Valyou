use std::cmp::min;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchMethod {
    Before,
    After
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub id: i64,
    #[serde(default = "default_limit")]
    pub limit: i64
}

#[derive(Debug, Serialize)]
#[serde(bound(serialize = "T: Serialize"))]
pub struct Paginated<T> {
    pub values: Vec<T>,
    pub pagination: Pagination
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub next_url: String
}

impl SearchQuery {
    pub fn into_parts(self) -> (i64, i64) {
        (if self.id < 0 { std::i64::MAX } else { self.id }, min(self.limit, 30))
    }
}

impl<T> Paginated<T> {
    pub fn paginate(values: Vec<T>, method: SearchMethod) -> Self {
        Paginated {
            values,
            pagination: Pagination {
                next_url: serde_json::to_string(&method).unwrap()
            }
        }
    }
}

#[inline(always)]
const fn default_limit() -> i64 { 20 }