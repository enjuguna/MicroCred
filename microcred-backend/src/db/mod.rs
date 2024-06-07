use mongodb::{Client, Collection, Database};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DID {
    pub id: String,
    pub credential: String,
}

pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    pub async fn init(connection_string: &str) -> Self {
        let client_options = ClientOptions::parse(connection_string).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("microcred");
        MongoDB { db }
    }

    pub fn get_collection(&self) -> Collection<DID> {
        self.db.collection::<DID>("microcred")
    }

    pub async fn create_did(&self, did: DID) -> mongodb::error::Result<()> {
        let collection = self.get_collection();
        collection.insert_one(did, None).await?;
        Ok(())
    }

    pub async fn get_did(&self, id: &str) -> mongodb::error::Result<Option<DID>> {
        let collection = self.get_collection();
        let filter = doc! { "id": id };
        let did = collection.find_one(filter, None).await?;
        Ok(did)
    }

    pub async fn update_did(&self, id: &str, new_credential: &str) -> mongodb::error::Result<()> {
        let collection = self.get_collection();
        let filter = doc! { "id": id };
        let update = doc! { "$set": { "credential": new_credential } };
        collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn delete_did(&self, id: &str) -> mongodb::error::Result<()> {
        let collection = self.get_collection();
        let filter = doc! { "id": id };
        collection.delete_one(filter, None).await?;
        Ok(())
    }
}
