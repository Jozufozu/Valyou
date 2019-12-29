use serde::Serialize;

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
    pub fn paginate(values: Vec<T>) -> Self {
        Paginated {
            values,
            pagination: Pagination {
                next_url: "".into()
            }
        }
    }
}