use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PaginationParams {
    pub page: u64,
    pub size: u64,
}
