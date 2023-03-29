use mongodb::{bson::doc, Client};

use crate::{
    errors::WebError,
    models::users::{CreateUser, User},
    utils::token::token_generator,
};

/**
 * Get the user collection from the database
 * @param database The database client
 */
pub fn serv_user_database(database: &Client) -> mongodb::Collection<User> {
    // TODO: Convert it into a real database
    database.database("test").collection("users")
}

/**
 * Register a new user
 * @param database The database client
 * @param user_info The user information
 *
 * @return The token of the user
 *
 * @throws WebError::DBError
 *
 * @note The token is generated using nanoid
 */
pub async fn serv_user_register(
    database: &Client,
    user_info: CreateUser,
) -> Result<String, WebError> {
    let users: mongodb::Collection<User> = serv_user_database(database);
    let mut user = User::from(user_info.clone());
    // let db automatically generate the id
    user._id = Some(bson::oid::ObjectId::new());

    users.insert_one(user, None).await?;

    serv_user_login(database, user_info).await
}

/**
 * Login a user
 * @param database The database client
 * @param user_info The user information
 *
 * @return The token of the user
 */
pub async fn serv_user_login(database: &Client, user_info: CreateUser) -> Result<String, WebError> {
    let users: mongodb::Collection<User> = serv_user_database(database);
    let token = token_generator();

    let res = users
        .update_one(
            doc! {"username": user_info.username, "password": user_info.password, "is_deprecated": false},
            doc! {"$set": {"token": token.clone()}},
            None,
        )
        .await?;

    if res.modified_count == 0 {
        return Err(WebError::DBError("User not found".to_string()));
    }

    Ok(token)
}

/**
 * Logout a user
 * @param database The database client
 * @param token_ The token of the user
 *
 */
pub async fn serv_user_logout(database: &Client, token_: String) -> Result<(), WebError> {
    let users: mongodb::Collection<User> = serv_user_database(database);
    users
        .update_one(doc! {"token": token_}, doc! {"$set": {"token": ""}}, None)
        .await?;
    Ok(())
}

/**
 * Get the user profile
 * @param database The database client
 * @param username The username of the user
 *
 * @return The user profile
 */
pub async fn serv_user_profile(database: &Client, username: String) -> Result<User, WebError> {
    let users: mongodb::Collection<User> = serv_user_database(database);

    let mut user_profile = users
        .find_one(doc! {"username": username}, None)
        .await
        .map(|user| user.unwrap())?;

    user_profile.password = "".to_string();
    user_profile.token = "".to_string();
    Ok(user_profile)
}

/**
 * Update the user profile
 * @param database The database client
 * @param user_info The user information
 *
 * @return The user profile
 */
pub async fn serv_user_update(database: &Client, mut user_info: User) -> Result<User, WebError> {
    let users: mongodb::Collection<User> = serv_user_database(database);

    let user_profile = users
        .find_one(doc! {"username": user_info.username.clone()}, None)
        .await
        .map(|user| user.unwrap())?;

    if user_info.token != user_profile.token {
        return Err(WebError::DBError("Token error".into()));
    }

    users
        .update_one(
            doc! {"username": user_info.username.clone()},
            doc! {"$set": &user_info},
            None,
        )
        .await?;

    user_info.password = "".to_string();
    user_info.token = "".to_string();

    Ok(user_info)
}

/**
 * Delete the user profile
 * @param database The database client
 * @param username The username of the user
 *
 */
pub async fn serv_user_delete(database: &Client, username: String) -> Result<(), WebError> {
    let users: mongodb::Collection<User> = serv_user_database(database);

    let res = users
        .update_one(
            doc! {"username": username},
            doc! {"$set": {"is_deprecated": true}},
            None,
        )
        .await?;

    if res.modified_count == 0 {
        return Err(WebError::DBError("User not found".to_string()));
    }

    Ok(())
}
