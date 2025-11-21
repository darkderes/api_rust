use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bson::oid::ObjectId;
use mongodb::bson::doc;

use crate::{
    db::mongo::{obtener_coleccion_tareas, DbState},
    models::tarea::{ActualizarTareaRequest, CrearTareaRequest, Tarea},
};

// POST /tareas - Crear una nueva tarea
pub async fn crear_tarea(
    State(db): State<DbState>,
    Json(payload): Json<CrearTareaRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let coleccion = obtener_coleccion_tareas(&db);
    
    let nueva_tarea = Tarea::nueva(payload.descripcion, payload.estado);
    
    match coleccion.insert_one(nueva_tarea, None).await {
        Ok(result) => {
            let id = result.inserted_id.as_object_id().unwrap();
            match coleccion.find_one(doc! { "_id": id }, None).await {
                Ok(Some(tarea)) => Ok((StatusCode::CREATED, Json(tarea))),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// GET /tareas - Listar todas las tareas
pub async fn listar_tareas(
    State(db): State<DbState>,
) -> Result<impl IntoResponse, StatusCode> {
    let coleccion = obtener_coleccion_tareas(&db);
    
    use futures::stream::StreamExt;
    
    match coleccion.find(None, None).await {
        Ok(mut cursor) => {
            let tareas: Vec<Tarea> = cursor
                .filter_map(|result| async move { result.ok() })
                .collect()
                .await;
            Ok(Json(tareas))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// GET /tareas/:id - Obtener una tarea por ID
pub async fn obtener_tarea(
    State(db): State<DbState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let coleccion = obtener_coleccion_tareas(&db);
    
    let object_id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    match coleccion.find_one(doc! { "_id": object_id }, None).await {
        Ok(Some(tarea)) => Ok(Json(tarea)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// PUT /tareas/:id - Actualizar una tarea
pub async fn actualizar_tarea(
    State(db): State<DbState>,
    Path(id): Path<String>,
    Json(payload): Json<ActualizarTareaRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let coleccion = obtener_coleccion_tareas(&db);
    
    let object_id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mut update_doc = doc! {};
    
    if let Some(descripcion) = payload.descripcion {
        update_doc.insert("descripcion", descripcion);
    }
    
    if let Some(estado) = payload.estado {
        update_doc.insert("estado", bson::to_bson(&estado).unwrap());
    }
    
    if update_doc.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    match coleccion
        .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc }, None)
        .await
    {
        Ok(result) if result.matched_count > 0 => {
            match coleccion.find_one(doc! { "_id": object_id }, None).await {
                Ok(Some(tarea)) => Ok(Json(tarea)),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// DELETE /tareas/:id - Eliminar una tarea
pub async fn eliminar_tarea(
    State(db): State<DbState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let coleccion = obtener_coleccion_tareas(&db);
    
    let object_id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    match coleccion.delete_one(doc! { "_id": object_id }, None).await {
        Ok(result) if result.deleted_count > 0 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
