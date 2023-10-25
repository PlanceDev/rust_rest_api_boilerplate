use crate::database::db::AppState;
use crate::errors::errors::Error;

use chrono::{DateTime, Utc};
use rcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Transaction};
use std::sync::Arc;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct GetUserParams {
    pub id: Uuid,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedUser {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct SearchUser {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, display_name: String, email: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            display_name,
            email,
            password_hash: hash(password, DEFAULT_COST).unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn get_password(&self) -> &Vec<u8> {
        &self.password_hash
    }

    pub fn set_password(&mut self, password: String) {
        self.password_hash = hash(password, DEFAULT_COST).unwrap();
    }

    pub fn verify_password(&self, password: String) -> bool {
        verify(password, &self.password_hash).unwrap()
    }

    pub async fn save(&self, session: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (id, username, display_name, email, password_hash) VALUES ($1, $2, $3, $4, $5)",
            self.id,
            self.username,
            self.display_name,
            self.email,
            self.password_hash
        )
        .execute(session)
        .await
        .map_err(|e| Error::CreateUserError("Could not create account.".to_string()))?;

        Ok(())
    }
}
