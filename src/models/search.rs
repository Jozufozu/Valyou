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
pub struct Paginated<T: Serialize> {
    pub values: Vec<T>,
    pub pagination: Pagination
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub next_url: String
}

impl<T: Serialize> Paginated<T> {
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