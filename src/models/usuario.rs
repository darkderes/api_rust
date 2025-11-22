use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usuario {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String, // Hasheado con bcrypt
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_token_expire: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegistroRequest {
    #[validate(length(min = 2, message = "El nombre debe tener al menos 2 caracteres"))]
    pub name: String,
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    #[validate(length(min = 6, message = "La contraseña debe tener al menos 6 caracteres"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 6, message = "La contraseña debe tener al menos 6 caracteres"))]
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UsuarioPublico>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsuarioPublico {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl Usuario {
    pub fn nuevo(name: String, email: String, password_hash: String) -> Self {
        Self {
            id: None,
            name,
            email: email.to_lowercase(),
            password: password_hash,
            created_at: Utc::now(),
            reset_token: None,
            reset_token_expire: None,
        }
    }

    pub fn to_publico(&self) -> UsuarioPublico {
        UsuarioPublico {
            id: self.id.as_ref().map(|id| id.to_hex()).unwrap_or_default(),
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}
