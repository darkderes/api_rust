use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tarea {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub descripcion: String,
    pub fecha_creacion: DateTime<Utc>,
    pub estado: EstadoTarea,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EstadoTarea {
    Pendiente,
    Ejecucion,
    Realizada,
}

#[derive(Debug, Deserialize)]
pub struct CrearTareaRequest {
    pub descripcion: String,
    #[serde(default = "default_estado")]
    pub estado: EstadoTarea,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarTareaRequest {
    pub descripcion: Option<String>,
    pub estado: Option<EstadoTarea>,
}

fn default_estado() -> EstadoTarea {
    EstadoTarea::Pendiente
}

impl Tarea {
    pub fn nueva(descripcion: String, estado: EstadoTarea) -> Self {
        Self {
            id: None,
            descripcion,
            fecha_creacion: Utc::now(),
            estado,
        }
    }
}
