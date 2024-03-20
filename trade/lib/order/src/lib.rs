use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    results::InsertOneResult,
    Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub size: f64,
    pub bid_or_ask: BidOrAsk,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order {
            id: None,
            bid_or_ask,
            size,
            created_at: Some(DateTime::now()),
        }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }

    // CRUD operations
    pub async fn save(
        &self,
        collection: &Collection<Order>,
    ) -> mongodb::error::Result<InsertOneResult> {
        collection.insert_one(self, None).await
    }

    pub async fn find_by_id(
        collection: &Collection<Order>,
        order_id: ObjectId,
    ) -> mongodb::error::Result<Option<Order>> {
        collection.find_one(doc! { "_id": order_id }, None).await
    }

    pub async fn update_size(
        &mut self,
        collection: &Collection<Order>,
        new_size: f64,
    ) -> mongodb::error::Result<()> {
        if let Some(id) = self.id {
            collection
                .update_one(
                    doc! { "_id": id },
                    doc! { "$set": { "size": new_size }},
                    None,
                )
                .await?;
            self.size = new_size;
        }
        Ok(())
    }

    pub async fn delete(
        collection: &Collection<Order>,
        order_id: ObjectId,
    ) -> mongodb::error::Result<()> {
        let delete_result = collection
            .delete_one(doc! { "_id": order_id }, None)
            .await?;

        if delete_result.deleted_count == 0 {
            println!("Document failed to be deleted. ID: {}", order_id);
        } else {
            println!("Document deleted successfully. ID: {}", order_id);
        }

        Ok(())
    }
}
