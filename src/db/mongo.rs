use mongodb::{bson::doc, Client, Collection, Database};
use crate::models::tarea::Tarea;

pub type DbState = Database;

pub async fn conectar_mongodb() -> Result<Database, mongodb::error::Error> {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017/".to_string());
    let client = Client::with_uri_str(&uri).await?;
    
    // Verificar conexión
    client
        .database("admin")
        .run_command(doc! { "ping": 1 }, None)
        .await?;
    
    let db = client.database("todo_db");
    Ok(db)
}

pub fn obtener_coleccion_tareas(db: &Database) -> Collection<Tarea> {
    db.collection::<Tarea>("tareas")
}
