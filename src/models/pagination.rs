use serde::Deserialize;

pub fn default_page() -> i64 {
    1
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: i64,
}

pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total_count: i64, page: i64, per_page: i64) -> Self {
        let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
        let total_pages = total_pages.max(1);
        Self {
            items,
            page,
            total_pages,
        }
    }

    pub fn has_prev(&self) -> bool {
        self.page > 1
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }
}
