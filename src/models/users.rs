use actix_web::web;
use bson::Bson;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Education {
    Bachelor,
    Master,
    Doctor,
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub _id: Option<bson::oid::ObjectId>,
    // basic info
    pub username: String,
    pub password: String,
    pub gender: Gender,
    pub education: Education,
    pub description: String,
    pub avatar: String,

    // optional info
    pub school: String,
    pub major: String,
    pub phone: String,
    pub email: String,

    // list info
    pub following: Vec<String>,
    pub participated: Vec<String>,
    pub published: Vec<String>,
    pub collection: Vec<String>,

    // register time
    pub register_time: i64,

    // token
    pub token: String,

    // is deprecated
    pub is_deprecated: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl From<web::Json<CreateUser>> for CreateUser {
    fn from(value: web::Json<CreateUser>) -> Self {
        CreateUser {
            username: value.username.clone(),
            password: value.password.clone(),
            phone: value.phone.clone(),
            email: value.email.clone(),
        }
    }
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        User {
            _id: None,
            username: value.username,
            password: value.password,
            gender: Gender::Other,
            education: Education::Other,
            description: String::new(),
            avatar: String::new(),
            school: String::new(),
            major: String::new(),
            phone: String::new(),
            email: String::new(),
            following: vec![],
            participated: vec![],
            published: vec![],
            collection: vec![],
            register_time: Utc::now().timestamp(),
            token: String::new(),
            is_deprecated: false,
        }
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Other => write!(f, "Other"),
        }
    }
}

impl std::fmt::Display for Education {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Education::Bachelor => write!(f, "Bachelor"),
            Education::Master => write!(f, "Master"),
            Education::Doctor => write!(f, "Doctor"),
            Education::Other => write!(f, "Other"),
        }
    }
}

impl std::convert::From<Gender> for Bson {
    fn from(value: Gender) -> Self {
        value.to_string().into()
    }
}

impl std::convert::From<Education> for Bson {
    fn from(value: Education) -> Self {
        value.to_string().into()
    }
}

impl std::convert::From<User> for Bson {
    fn from(value: User) -> Self {
        let mut doc = bson::Document::new();
        doc.insert("username", value.username);
        doc.insert("password", value.password);
        doc.insert("gender", value.gender);
        doc.insert("education", value.education);
        doc.insert("description", value.description);
        doc.insert("avatar", value.avatar);
        doc.insert("school", value.school);
        doc.insert("major", value.major);
        doc.insert("phone", value.phone);
        doc.insert("email", value.email);
        doc.insert("following", value.following);
        doc.insert("participated", value.participated);
        doc.insert("published", value.published);
        doc.insert("collection", value.collection);
        doc.insert("register_time", value.register_time);
        doc.insert("token", value.token);
        doc.insert("is_deprecated", value.is_deprecated);
        Bson::Document(doc)
    }
}
