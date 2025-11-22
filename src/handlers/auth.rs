use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::bson::doc;
use validator::Validate;
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};

use crate::{
    db::mongo::{obtener_coleccion_usuarios, DbState},
    models::usuario::{
        AuthResponse, ForgotPasswordRequest, LoginRequest, RegistroRequest,
        ResetPasswordRequest, Usuario,
    },
    utils::jwt::generar_token,
};

// POST /auth/register - Registrar nuevo usuario
pub async fn registro(
    State(db): State<DbState>,
    Json(payload): Json<RegistroRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validar datos
    if let Err(_) = payload.validate() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(AuthResponse {
                success: false,
                token: None,
                user: None,
                message: "Datos inválidos".to_string(),
            }),
        ));
    }

    let coleccion = obtener_coleccion_usuarios(&db);

    // Verificar si el email ya existe
    let email_lower = payload.email.to_lowercase();
    match coleccion
        .find_one(doc! { "email": &email_lower }, None)
        .await
    {
        Ok(Some(_)) => {
            return Ok((
                StatusCode::BAD_REQUEST,
                Json(AuthResponse {
                    success: false,
                    token: None,
                    user: None,
                    message: "El email ya está registrado".to_string(),
                }),
            ));
        }
        Ok(None) => {}
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    // Hashear contraseña
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Crear usuario
    let nuevo_usuario = Usuario::nuevo(payload.name, email_lower, password_hash);

    // Insertar en base de datos
    match coleccion.insert_one(&nuevo_usuario, None).await {
        Ok(result) => {
            let user_id = result.inserted_id.as_object_id().unwrap();

            // Generar token JWT
            let token = match generar_token(&user_id.to_hex()) {
                Ok(t) => t,
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };

            // Obtener usuario creado
            match coleccion.find_one(doc! { "_id": user_id }, None).await {
                Ok(Some(usuario)) => Ok((
                    StatusCode::CREATED,
                    Json(AuthResponse {
                        success: true,
                        token: Some(token),
                        user: Some(usuario.to_publico()),
                        message: "Usuario registrado exitosamente".to_string(),
                    }),
                )),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// POST /auth/login - Iniciar sesión
pub async fn login(
    State(db): State<DbState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validar datos
    if let Err(_) = payload.validate() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(AuthResponse {
                success: false,
                token: None,
                user: None,
                message: "Datos inválidos".to_string(),
            }),
        ));
    }

    let coleccion = obtener_coleccion_usuarios(&db);
    let email_lower = payload.email.to_lowercase();

    // Buscar usuario por email
    match coleccion
        .find_one(doc! { "email": &email_lower }, None)
        .await
    {
        Ok(Some(usuario)) => {
            // Verificar contraseña
            match verify(&payload.password, &usuario.password) {
                Ok(true) => {
                    // Generar token JWT
                    let user_id = usuario.id.as_ref().unwrap().to_hex();
                    let token = match generar_token(&user_id) {
                        Ok(t) => t,
                        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                    };

                    Ok((
                        StatusCode::OK,
                        Json(AuthResponse {
                            success: true,
                            token: Some(token),
                            user: Some(usuario.to_publico()),
                            message: "Inicio de sesión exitoso".to_string(),
                        }),
                    ))
                }
                Ok(false) => Ok((
                    StatusCode::UNAUTHORIZED,
                    Json(AuthResponse {
                        success: false,
                        token: None,
                        user: None,
                        message: "Credenciales incorrectas".to_string(),
                    }),
                )),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Ok((
            StatusCode::UNAUTHORIZED,
            Json(AuthResponse {
                success: false,
                token: None,
                user: None,
                message: "Credenciales incorrectas".to_string(),
            }),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// POST /auth/forgot-password - Solicitar reseteo de contraseña
pub async fn forgot_password(
    State(db): State<DbState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validar datos
    if let Err(_) = payload.validate() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": "Email inválido"
            })),
        ));
    }

    let coleccion = obtener_coleccion_usuarios(&db);
    let email_lower = payload.email.to_lowercase();

    // Buscar usuario por email
    match coleccion
        .find_one(doc! { "email": &email_lower }, None)
        .await
    {
        Ok(Some(usuario)) => {
            // Generar token de reseteo aleatorio
            let reset_token: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();

            // Establecer expiración de 24 horas
            let expire_at = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("Fecha válida");

            // Actualizar usuario con token de reseteo
            let update_doc = doc! {
                "$set": {
                    "reset_token": &reset_token,
                    "reset_token_expire": expire_at
                }
            };

            match coleccion
                .update_one(
                    doc! { "_id": usuario.id.unwrap() },
                    update_doc,
                    None,
                )
                .await
            {
                Ok(_) => Ok((
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "success": true,
                        "message": "Token de reseteo generado",
                        "reset_token": reset_token,
                        "note": "En producción, este token se enviaría por email"
                    })),
                )),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Ok((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "success": false,
                "message": "Email no encontrado"
            })),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// POST /auth/reset-password - Resetear contraseña con token
pub async fn reset_password(
    State(db): State<DbState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validar datos
    if let Err(_) = payload.validate() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": "Datos inválidos"
            })),
        ));
    }

    let coleccion = obtener_coleccion_usuarios(&db);

    // Buscar usuario por token de reseteo
    match coleccion
        .find_one(doc! { "reset_token": &payload.token }, None)
        .await
    {
        Ok(Some(usuario)) => {
            // Verificar que el token no haya expirado
            if let Some(expire_at) = usuario.reset_token_expire {
                if Utc::now() > expire_at {
                    return Ok((
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({
                            "success": false,
                            "message": "El token ha expirado"
                        })),
                    ));
                }
            } else {
                return Ok((
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "success": false,
                        "message": "Token inválido"
                    })),
                ));
            }

            // Hashear nueva contraseña
            let password_hash = match hash(&payload.new_password, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };

            // Actualizar contraseña y limpiar token de reseteo
            let update_doc = doc! {
                "$set": {
                    "password": password_hash
                },
                "$unset": {
                    "reset_token": "",
                    "reset_token_expire": ""
                }
            };

            match coleccion
                .update_one(
                    doc! { "_id": usuario.id.unwrap() },
                    update_doc,
                    None,
                )
                .await
            {
                Ok(_) => Ok((
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "success": true,
                        "message": "Contraseña actualizada exitosamente"
                    })),
                )),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Ok((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "success": false,
                "message": "Token inválido"
            })),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
