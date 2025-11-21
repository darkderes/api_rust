mod db;
mod handlers;
mod models;
mod routes;

use axum::Router;
use db::mongo::conectar_mongodb;
use routes::crear_rutas;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Conectar a MongoDB
    let db = conectar_mongodb().await.expect("Error al conectar con MongoDB");
    println!("✓ Conectado a MongoDB");

    // Crear rutas con el estado compartido
    use tower_http::cors::Any;
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        .merge(crear_rutas())
        .layer(cors)
        .with_state(db);

    // Configurar dirección del servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Servidor ejecutándose en http://{}", addr);

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Error al crear listener");
    
    axum::serve(listener, app)
        .await
        .expect("Error al ejecutar servidor");
}
