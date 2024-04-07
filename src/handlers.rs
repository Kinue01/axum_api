use axum::{ extract::{ State, Json }, http::StatusCode };
use sqlx::PgPool;

use crate::{ 
    models::{ UserFromDb, User, RoleFromDb, UserInfo, UserRegister, Item, ItemFromDb, ItemType, ItemInfo }, 
    errors::MyError,
};

pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, MyError> {
    
    let users: Vec<UserFromDb> = sqlx::query_as("select * from tb_user order by user_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    let roles: Vec<RoleFromDb> = sqlx::query_as("select * from tb_role")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    let mut res: Vec<User> = Vec::new();

    for user in &users {
        res.push(
            User 
            { 
                user_id: user.user_id, 
                user_login: user.user_login.clone(), 
                user_password: user.user_password.clone(), 
                user_role: roles.iter().find(|&role| role.role_id == user.user_role_id).unwrap().role_name.clone(), 
                user_email: user.user_email.clone() 
            }
        )
    }

    Ok(Json(res))
    
}

pub async fn add_user(State(pool): State<PgPool>, Json(user): Json<UserRegister>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("insert into tb_user (user_login, user_password, user_email) values ($1, $2, $3)")
    .bind(&user.user_login).bind(&user.user_password).bind(&user.user_email)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::CREATED)

}

pub async fn get_user(State(pool): State<PgPool>, Json(user): Json<UserInfo>) -> Result<Json<Vec<User>>, MyError> {

    let users: Vec<UserFromDb> = sqlx::query_as("select * from tb_user order by user_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    let roles: Vec<RoleFromDb> = sqlx::query_as("select * from tb_role")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    let mut res: Vec<User> = Vec::new();

    for user in &users {
        res.push(
            User 
            { 
                user_id: user.user_id, 
                user_login: user.user_login.clone(), 
                user_password: user.user_password.clone(), 
                user_role: roles.iter().find(|&role| role.role_id == user.user_role_id).unwrap().role_name.clone(), 
                user_email: user.user_email.clone() 
            }
        )
    }

    let is_user: Vec<User> = res.into_iter()
    .filter(|usr| usr.user_login == user.user_login && usr.user_password == user.user_password).collect();

    Ok(Json(is_user))

}

pub async fn get_items(State(pool): State<PgPool>) -> Result<Json<Vec<Item>>, MyError> {

    let items: Vec<ItemFromDb> = sqlx::query_as("select * from tb_item order by item_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    let types: Vec<ItemType> = sqlx::query_as("select * from tb_item_type")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    let mut res: Vec<Item> = Vec::new();

    for item in &items {
        res.push(Item {
            item_id: item.item_id,
            item_name: item.item_name.clone(),
            item_description: item.item_description.clone(),
            item_type: types.iter().find(|&typ| typ.type_id == item.item_type_id).unwrap().type_name.clone(),
            item_image: item.item_image.clone()
        })
    }

    Ok(Json(res))

}


pub async fn add_item(State(pool): State<PgPool>, Json(item): Json<ItemInfo>) -> Result<StatusCode, MyError> {

    let types: Vec<ItemType> = sqlx::query_as("select * from tb_item_type")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    sqlx::query("insert into tb_item (item_name, item_description, item_type_id, item_image) values ($1, $2, $3, './img/default.png')")
    .bind(&item.item_name).bind(&item.item_description).bind(types.iter().find(|imt| imt.type_name == item.item_type).unwrap().type_id)
    .execute(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(StatusCode::CREATED)

}

pub async fn get_item_types(State(pool): State<PgPool>) -> Result<Json<Vec<ItemType>>, MyError> {

    let types: Vec<ItemType> = sqlx::query_as("select * from tb_item_type")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(types))

}