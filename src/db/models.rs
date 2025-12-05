use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RacialTrait {
    pub id: surrealdb::sql::Thing,
    pub trait_name: String,
    pub description: String,
}
