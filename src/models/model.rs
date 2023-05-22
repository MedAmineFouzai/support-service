use super::support::Support;
use bson::{doc, oid::ObjectId, Document};
use core::fmt::Debug;
use load_dotenv::load_dotenv;
use mongodb::{
    error::Error,
    options::{ClientOptions, FindOneAndUpdateOptions, ReturnDocument},
    Client, Collection, Cursor,
};
use std::{borrow::Borrow, str::FromStr};

#[derive(Debug, Clone)]
pub struct SupportCollection {
    collection: Collection<Support>,
}

impl SupportCollection {
    pub fn new(collection: Collection<Support>) -> SupportCollection {
        SupportCollection { collection }
    }

    pub async fn connect() -> Collection<Support> {
        load_dotenv!();
        let client_options = ClientOptions::parse(env!("DATABASE_URL")).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(env!("SUPPORT_DATABASE"));
        db.collection(env!("SUPPORT_COLLECTION"))
      
    }

    pub async fn find_one<T>(&self, document: T) -> Result<Option<Support>, Error>
    where
        T: serde::Serialize + Debug,
    {
        println!("{:?}", document);
        let query = match bson::to_bson(&document) {
            Ok(bson_document) => match bson_document.as_document() {
                Some(document) => document.clone(),
                None => Document::new(),
            },
            _ => Document::new(),
        };
        dbg!(&query);

        return self.collection.find_one(query, None).await;
    }

    pub async fn find_all(&self, query: Document) -> Result<Cursor<Support>, Error> {
        Ok(self.collection.find(query, None).await?)
    }

    pub async fn delete_one(&self, thread_id: &str) -> Result<Option<Support>, Error> {
        Ok(self
            .collection
            .find_one_and_delete(
                doc! {
                "_id":match ObjectId::from_str(thread_id){
                    Ok(user_id)=>user_id,
                    Err(_)=>ObjectId::new()
                }                   },
                None,
            )
            .await?)
    }

    pub async fn insert_one<T>(&self, document: T) -> Result<(), Error>
    where
        T: serde::Serialize + Debug + Borrow<Support> + Clone,
    {
        self.collection.insert_one(document, None).await?;
        Ok(())
    }

    pub async fn add_message<T>(
        &self,
        thread_id: &str,
        document: T,
    ) -> Result<Option<Support>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":match ObjectId::from_str(thread_id){
                        Ok(user_id)=>user_id,
                        Err(_)=>ObjectId::new()
                    }
                },
                doc! {
                  "$push":{
                    "user_messages":match bson::to_bson(&document) {
                        Ok(bson_document) => match bson_document.as_document() {
                            Some(document) => document.clone(),
                            None => Document::new(),
                        },
                        _ => Document::new(),
                    }
                  }
                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }
}
