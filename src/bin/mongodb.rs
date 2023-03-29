use mongodb::{bson::doc, bson::Document, options::ClientOptions, Collection};
use std::error::Error;
use mlum_inner::models::users::User;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a client that connects to the computer_scientists collection
    let options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = mongodb::Client::with_options(options)?;
    let users: Collection<User> = client.database("test").collection("users");

    let user = users.find_one(doc! {"username": "dessera"}, None).await?;
    print!("{:?}", user.unwrap());
    Ok(())
}
