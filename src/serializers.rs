use serde::{Deserialize, Serialize};

use crate::database::models::NewPerson;

#[derive(Debug, Deserialize)]
pub struct PersonIn {
    pub name: String,
    pub dept: String,
    pub role: String,
    pub email: String,
    pub currency: String,
}

impl Into<NewPerson> for PersonIn {
    fn into(self) -> NewPerson {
        NewPerson {
            email: self.email,
            name: self.name,
            role: self.role,
            currency: self.currency,
            dept: self.dept,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PersonOut {
    pub name: String,
    pub dept: String,
    pub role: String,
    pub email: String,
    pub currency: String,
    pub created: bool,
    pub balance: f32,
    pub value: f32,
}

impl PersonOut {
    pub fn fields(&self) -> Vec<String> {
        vec![
            "name".to_string(),
            "dept".to_string(),
            "role".to_string(),
            "email".to_string(),
            "currency".to_string(),
            "created".to_string(),
            "balance".to_string(),
            "value".to_string(),
        ]
    }

    pub fn get(&self, key: &str) -> String {
        match key {
            "name" => self.name.clone(),
            "dept" => self.dept.clone(),
            "role" => self.role.clone(),
            "email" => self.email.clone(),
            "currency" => self.currency.clone(),
            "created" => self.created.to_string(),
            "balance" => self.balance.to_string(),
            "value" => self.value.to_string(),
            //TODO: create an error
            _ => "".to_string(),
        }
    }
}
