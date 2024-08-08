use mongodb::{bson::{binary::Error, doc, oid::ObjectId, Document}, options::{ClientOptions, ServerApi, ServerApiVersion}, Client, Collection, Cursor, Database};

use crate::models::Test;


pub struct Repo {
    client: Client,
    database: Database,
    test_collection: Collection<Test>
}

impl Repo {
    pub async fn new(url: &str) -> Repo {
        let mut client_options = ClientOptions::parse(url).await.unwrap();

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        
        let client = Client::with_options(client_options).unwrap();
        
        Self { 
            client: client.clone(),
            database: client.database("invemanage"),
            test_collection: client.database("invemanage").collection("test")
        }
    }

    pub async fn test_impl(&self) -> Cursor<Test> {
        let result: Cursor<Test> = self
            .test_collection
            .find( doc! {} )
            .await
            .unwrap();
        return result;
    }
}