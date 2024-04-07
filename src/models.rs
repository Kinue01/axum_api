use serde::{Deserialize, Serialize};
use sqlx::prelude::*;


#[derive(Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct UserFromDb{
    pub user_id: i32,
    pub user_login: String,
    pub user_password: String,
    pub user_role_id: i32,
    pub user_email: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub user_id: i32,
    pub user_login: String,
    pub user_password: String,
    pub user_role: String,
    pub user_email: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserRegister {
    pub user_login: String,
    pub user_password: String,
    pub user_email: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub user_login: String,
    pub user_password: String
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct RoleFromDb {
    pub role_id: i32,
    pub role_name: String
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct ItemFromDb {
    pub item_id: i32,
    pub item_name: String,
    pub item_description: String,
    pub item_type_id: i32,
    pub item_image: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub item_id: i32,
    pub item_name: String,
    pub item_description: String,
    pub item_type: String,
    pub item_image: String
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct ItemType {
    pub type_id: i32,
    pub type_name: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemInfo {
    pub item_name: String,
    pub item_description: String,
    pub item_type: String
}