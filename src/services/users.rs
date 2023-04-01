use actix_web::http::StatusCode;
use chrono::Utc;
use mongodb::{bson::doc, Client};

use crate::{
    errors::WebError,
    models::users::{CertificateUser, CreateUser, User},
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
 * Verify the user token
 * @param database The database client
 * @param username The username of the user
 *
 * @return The data of the user
 */
pub async fn serv_user_token_verify(database: &Client, username: String) -> Result<User, WebError> {
    let users: mongodb::Collection<User> = serv_user_database(database);
    let user = users
        .find_one(doc! {"username": username.clone()}, None)
        .await
        .map(|user| user.unwrap())?;

    if user.token == "" {
        return Err(WebError::new(
            StatusCode::UNAUTHORIZED,
            "You need to login first!".to_string(),
        ));
    }

    if user.valid_token_time < Utc::now().timestamp() {
        users
            .update_one(
                doc! {"username": username},
                doc! {"$set": {"token": ""}},
                None,
            )
            .await?;
        return Err(WebError::new(
            StatusCode::UNAUTHORIZED,
            "Token expired!".to_string(),
        ));
    }

    Ok(user)
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
            doc! {"$set": {"token": token.clone(), "valid_token_time": Utc::now().timestamp() + 3600}},
            None,
        )
        .await?;

    if res.modified_count == 0 {
        return Err(WebError::new(
            StatusCode::UNAUTHORIZED,
            "Username or password error!".to_string(),
        ));
    }

    Ok(token)
}

/**
 * Logout a user
 * @param database The database client
 * @param token_ The token of the user
 *
 */
pub async fn serv_user_logout(database: &Client, token: String) -> Result<(), WebError> {
    let users: mongodb::Collection<User> = serv_user_database(database);
    let res = users
        .update_one(
            doc! {"token": token},
            doc! {"$set": {"token": "", "valid_token_time": 0}},
            None,
        )
        .await?;
    if res.modified_count == 0 {
        return Err(WebError::new(
            StatusCode::UNAUTHORIZED,
            "Token expired!".to_string(),
        ));
    }

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

    let user_profile = serv_user_token_verify(database, user_info.username.clone()).await?;
    if user_info.token != user_profile.token {
        return Err(WebError::new(
            StatusCode::UNAUTHORIZED,
            "Token error!".to_string(),
        ));
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
 *
 */
pub async fn serv_user_delete(
    database: &Client,
    certification: CertificateUser,
) -> Result<(), WebError> {
    let users: mongodb::Collection<User> = serv_user_database(database);

    let res = users
        .update_one(
            doc! {"username": certification.username, "token": certification.token},
            doc! {"$set": {"is_deprecated": true}},
            None,
        )
        .await?;

    if res.modified_count == 0 {
        return Err(WebError::new(
            StatusCode::UNAUTHORIZED,
            "Username or token error!".to_string(),
        ));
    }

    Ok(())
}

/**
 * Verify the user
 * @param database The database client
 * @param certificate The certificate of the user
 */
pub async fn serv_user_verify(
    database: &Client,
    certificate: CertificateUser,
) -> Result<(), WebError> {
    let res = serv_user_token_verify(database, certificate.username.clone()).await?;

    if res.token != certificate.token {
        return Err(WebError::new(
            StatusCode::UNAUTHORIZED,
            "Token error!".to_string(),
        ));
    } else {
        Ok(())
    }
}
