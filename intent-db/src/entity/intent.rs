use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "intents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64, // IntentID from contract (casted to i64 for DB compatibility)
    
    #[sea_orm(indexed)]
    pub creator: String,
    
    pub input_token: String,
    pub input_amount: String, // Stored as string to preserve precision
    
    pub output_token: String,
    pub min_output_amount: String,
    
    pub deadline: i64,
    
    #[sea_orm(indexed)]
    pub status: i16, // 0=Pending, 1=Filled, 2=Cancelled
    
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
