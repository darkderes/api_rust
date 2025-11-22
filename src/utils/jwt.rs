use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

const JWT_SECRET: &str = "tu_clave_secreta_super_segura_cambiar_en_produccion";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,  // expiración
}

pub fn generar_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiracion = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("Fecha de expiración válida")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiracion,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

pub fn verificar_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
