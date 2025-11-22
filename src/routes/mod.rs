use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    db::mongo::DbState,
    handlers::tareas::{
        actualizar_tarea, crear_tarea, eliminar_tarea, listar_tareas, obtener_tarea,
    },
    handlers::auth::{
        registro, login, forgot_password, reset_password,
    },
};

pub fn crear_rutas() -> Router<DbState> {
    Router::new()
        // Rutas de tareas
        .route("/tareas", post(crear_tarea))
        .route("/tareas", get(listar_tareas))
        .route("/tareas/:id", get(obtener_tarea))
        .route("/tareas/:id", put(actualizar_tarea))
        .route("/tareas/:id", delete(eliminar_tarea))
        // Rutas de autenticación
        .route("/auth/register", post(registro))
        .route("/auth/login", post(login))
        .route("/auth/forgot-password", post(forgot_password))
        .route("/auth/reset-password", post(reset_password))
}
