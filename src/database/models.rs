use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_valid::Validate;

use crate::database::schema::balance;
use crate::database::schema::movement;
use crate::database::schema::person;
use crate::database::schema::user;
use crate::utils::email::email_validator;

#[derive(Queryable, Identifiable, AsChangeset, Clone, Debug)]
#[diesel(table_name = person)]
pub struct Person {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub role: String,
    pub currency: String,
    pub dept: String,
}

#[derive(Insertable, Validate)]
#[diesel(table_name = person)]
pub struct NewPerson {
    #[validate(custom(email_validator))]
    pub email: String,
    pub name: String,
    pub role: String,
    pub currency: String,
    pub dept: String,
}

#[derive(Queryable, Selectable, AsChangeset, Identifiable, Associations, Clone)]
#[diesel(belongs_to(Person))]
#[diesel(table_name = balance)]
pub struct Balance {
    pub id: i32,
    pub person_id: i32,
    pub value: f32,
}

#[derive(Insertable)]
#[diesel(table_name = balance)]
pub struct NewBalance {
    pub person_id: i32,
    pub value: f32,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Clone)]
#[diesel(belongs_to(Person))]
#[diesel(table_name = movement)]
pub struct Movement {
    pub id: i32,
    pub person_id: i32,
    pub value: f32,
    pub actor: String,
    pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = movement)]
pub struct NewMovement {
    pub person_id: i32,
    pub value: f32,
    pub actor: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Clone)]
#[diesel(belongs_to(Person))]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub password: String,
    pub person_id: i32,
    pub superuser: bool,
    pub username: String,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub password: String,
    pub person_id: i32,
    pub superuser: bool,
    pub username: String,
}
