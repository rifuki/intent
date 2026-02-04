use sea_orm::*;
use crate::entity::intent::{self, Entity as Intent};

#[async_trait::async_trait]
pub trait IntentRepository {
    async fn create(&self, data: intent::Model) -> Result<intent::Model, DbErr>;
    async fn find_by_id(&self, id: i64) -> Result<Option<intent::Model>, DbErr>;
    async fn find_by_creator(&self, creator: &str) -> Result<Vec<intent::Model>, DbErr>;
    async fn find_pending(&self) -> Result<Vec<intent::Model>, DbErr>;
    async fn update_status(&self, id: i64, status: i16) -> Result<Option<intent::Model>, DbErr>;
}

pub struct IntentRepoImpl {
    db: DatabaseConnection,
}

impl IntentRepoImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl IntentRepository for IntentRepoImpl {
    async fn create(&self, data: intent::Model) -> Result<intent::Model, DbErr> {
        let active_model = intent::ActiveModel {
            id: Set(data.id),
            creator: Set(data.creator),
            input_token: Set(data.input_token),
            input_amount: Set(data.input_amount),
            output_token: Set(data.output_token),
            min_output_amount: Set(data.min_output_amount),
            deadline: Set(data.deadline),
            status: Set(data.status),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        
        active_model.insert(&self.db).await
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<intent::Model>, DbErr> {
        Intent::find_by_id(id).one(&self.db).await
    }

    async fn find_by_creator(&self, creator: &str) -> Result<Vec<intent::Model>, DbErr> {
        Intent::find()
            .filter(intent::Column::Creator.eq(creator))
            .order_by_desc(intent::Column::CreatedAt)
            .all(&self.db)
            .await
    }
    
    async fn find_pending(&self) -> Result<Vec<intent::Model>, DbErr> {
        Intent::find()
             // Status 0 = Pending
            .filter(intent::Column::Status.eq(0)) 
            .order_by_asc(intent::Column::Deadline) // Earliest deadline first
            .all(&self.db)
            .await
    }
    
    async fn update_status(&self, id: i64, status: i16) -> Result<Option<intent::Model>, DbErr> {
         let intent = Intent::find_by_id(id).one(&self.db).await?;
         
         if let Some(intent) = intent {
             let mut active: intent::ActiveModel = intent.into();
             active.status = Set(status);
             active.updated_at = Set(chrono::Utc::now().naive_utc());
             let updated = active.update(&self.db).await?;
             Ok(Some(updated))
         } else {
             Ok(None)
         }
    }
}
