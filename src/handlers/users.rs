/**
 * route handlers for users
 */
use crate::{
    app_state,
    errors::WebError,
    models::users::{CertificateUser, CreateUser, User},
    services::users::*,
};

use actix_web::{web, HttpResponse};

pub async fn user_register(
    user_info: web::Json<CreateUser>,
    app_state: web::Data<app_state::AppState>,
) -> Result<HttpResponse, WebError> {
    serv_user_register(&app_state.database, user_info.into_inner())
        .await
        .map(|token| HttpResponse::Ok().json(token))
}

pub async fn user_login(
    user_info: web::Json<CreateUser>,
    app_state: web::Data<app_state::AppState>,
) -> Result<HttpResponse, WebError> {
    serv_user_login(&app_state.database, user_info.into_inner())
        .await
        .map(|token| HttpResponse::Ok().json(token))
}

pub async fn user_logout(
    token_: web::Json<String>,
    app_state: web::Data<app_state::AppState>,
) -> Result<HttpResponse, WebError> {
    serv_user_logout(&app_state.database, token_.into_inner())
        .await
        .map(|_| HttpResponse::Ok().json("logout success"))
}

pub async fn user_profile(
    user_name: web::Json<String>,
    app_state: web::Data<app_state::AppState>,
) -> Result<HttpResponse, WebError> {
    serv_user_profile(&app_state.database, user_name.into_inner())
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn user_update(
    user_info: web::Json<User>,
    app_state: web::Data<app_state::AppState>,
) -> Result<HttpResponse, WebError> {
    serv_user_update(&app_state.database, user_info.into_inner())
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn user_delete(
    certification: web::Json<CertificateUser>,
    app_state: web::Data<app_state::AppState>,
) -> Result<HttpResponse, WebError> {
    serv_user_delete(&app_state.database, certification.into_inner())
        .await
        .map(|_| HttpResponse::Ok().json("delete success"))
}

pub async fn user_verify(
    user_info: web::Json<CertificateUser>,
    app_state: web::Data<app_state::AppState>,
) -> Result<HttpResponse, WebError> {
    serv_user_verify(&app_state.database, user_info.into_inner())
        .await
        .map(|_| HttpResponse::Ok().json("certificate success"))
}

#[cfg(test)]
mod user_handler_test {
    use actix_web::{body::MessageBody, web};
    use tokio::sync::Mutex;

    use crate::models::users::{CertificateUser, CreateUser, User};

    async fn create_app_state() -> crate::app_state::AppState {
        let database = mongodb::Client::with_uri_str("mongodb://localhost:27017")
            .await
            .unwrap();
        crate::app_state::AppState {
            database,
            visit_count: Mutex::new(0),
            health_check_response: "I'm fine".to_string(),
        }
    }

    #[tokio::test]
    #[ignore = "username is not unique"]
    async fn test_user_register() {
        // make database link
        let app_state = create_app_state().await;
        // make user_info
        let user_info = web::Json(CreateUser {
            username: "dessera".into(),
            password: "123456".into(),
            phone: "".into(),
            email: "".into(),
        });
        // call user_register
        let result = super::user_register(user_info, web::Data::new(app_state)).await;
        // assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_login() {
        let app_state = create_app_state().await;
        let user_info = web::Json(CreateUser {
            username: "dessera".into(),
            password: "123456".into(),
            phone: "".into(),
            email: "".into(),
        });
        let result = super::user_login(user_info, web::Data::new(app_state)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_login_fail() {
        let app_state = create_app_state().await;
        let user_info = web::Json(CreateUser {
            username: "dessera".into(),
            password: "123456".into(),
            phone: "".into(),
            email: "".into(),
        });
        let result = super::user_login(user_info, web::Data::new(app_state)).await;
        assert!(!result.is_ok());
    }

    #[tokio::test]
    async fn test_user_logout() {
        let app_state = create_app_state().await;
        let user_info = web::Json(CreateUser {
            username: "dessera".into(),
            password: "123456".into(),
            phone: "".into(),
            email: "".into(),
        });

        // login first
        let result = super::user_login(user_info, web::Data::new(app_state)).await;
        assert!(result.is_ok());

        // get token
        let token = result.unwrap().into_body().try_into_bytes().unwrap().into();
        let token = String::from_utf8(token).unwrap();

        // remove the first and last "
        let token = token[1..token.len() - 1].to_string();

        // logout
        let app_state = create_app_state().await;
        let token = web::Json(token);
        let result = super::user_logout(token, web::Data::new(app_state)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_profile() {
        let app_state = create_app_state().await;
        let user_name = web::Json("dessera".into());
        let result = super::user_profile(user_name, web::Data::new(app_state)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "token is not valid because of login_test"]
    async fn test_user_update() {
        // login first
        let app_state = create_app_state().await;
        let user_info = web::Json(CreateUser {
            username: "dessera".into(),
            password: "123456".into(),
            phone: "".into(),
            email: "".into(),
        });
        let result = super::user_login(user_info, web::Data::new(app_state)).await;
        assert!(result.is_ok());

        // get token
        let token = result.unwrap().into_body().try_into_bytes().unwrap().into();
        let token = String::from_utf8(token).unwrap();
        // remove the first and last "
        let token = token[1..token.len() - 1].to_string();

        // get user info
        let app_state = create_app_state().await;
        let result = super::user_profile(
            web::Json(String::from("dessera")),
            web::Data::new(app_state),
        )
        .await;
        assert!(result.is_ok());

        // parse user info
        let user = result.unwrap().into_body().try_into_bytes().unwrap().into();
        let user = String::from_utf8(user).unwrap();
        let mut user: User = serde_json::from_str(&user).unwrap();
        user.token = token;
        user.password = String::from("123456");
        user.description = String::from("C++ programmer");

        // update user info
        let app_state = create_app_state().await;
        let result = super::user_update(web::Json(user), web::Data::new(app_state)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "do not delete user dessera"]
    async fn test_user_delete() {
        // login first
        let app_state = create_app_state().await;
        let user_info = web::Json(CreateUser {
            username: "dessera".into(),
            password: "123456".into(),
            phone: "".into(),
            email: "".into(),
        });
        let result = super::user_login(user_info, web::Data::new(app_state)).await;
        assert!(result.is_ok());

        // get token
        let token = result.unwrap().into_body().try_into_bytes().unwrap().into();
        let token = String::from_utf8(token).unwrap();
        // remove the first and last "
        let token = token[1..token.len() - 1].to_string();

        let app_state = create_app_state().await;
        let certification = web::Json(CertificateUser {
            username: String::from("dessera"),
            token,
        });

        let result = super::user_delete(certification, web::Data::new(app_state)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "token is not valid because of login_test"]
    async fn test_user_certificate() {
        // login first
        let app_state = create_app_state().await;
        let user_info = web::Json(CreateUser {
            username: "dessera".into(),
            password: "123456".into(),
            phone: "".into(),
            email: "".into(),
        });
        let result = super::user_login(user_info, web::Data::new(app_state)).await;
        assert!(result.is_ok());

        // get token
        let token = result.unwrap().into_body().try_into_bytes().unwrap().into();
        let token = String::from_utf8(token).unwrap();
        // remove the first and last "
        let token = token[1..token.len() - 1].to_string();

        // certificate
        let app_state = create_app_state().await;
        let certificate = web::Json(CertificateUser {
            username: String::from("dessera"),
            token,
        });
        let result = super::user_verify(certificate, web::Data::new(app_state)).await;
        assert!(result.is_ok());
    }
}
