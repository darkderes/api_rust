# Copilot Instructions - API Rust de Tareas

## Contexto del Proyecto

API REST en Rust para gestión de tareas (TODO list) con MongoDB como base de datos.

## Stack Tecnológico

- **Lenguaje**: Rust (no instalado aún en el equipo)
- **Base de Datos**: MongoDB en `mongodb://localhost:27017/`
- **Framework Web**: Axum (moderno, async/await nativo, excelente con tokio)
- **Driver MongoDB**: mongodb crate oficial
- **Sin autenticación**: Endpoints públicos sin auth

## Modelo de Datos

### Estructura de Tarea
```rust
struct Tarea {
    id: ObjectId,
    descripcion: String,
    fecha_creacion: DateTime<Utc>,  // Automática al crear
    estado: EstadoTarea,
}

enum EstadoTarea {
    Pendiente,
    Ejecucion,
    Realizada,
}
```

## Convenciones del Proyecto

- **Nombres en español**: Variables, structs y funciones usan español (descripcion, fecha_creacion, estado)
- **Estados de tarea**: Usar enum con tres estados exactos: Pendiente, Ejecucion, Realizada
- **Timestamps automáticos**: La fecha_creacion debe asignarse automáticamente en el servidor

## Setup y Comandos Clave

### Instalación de Rust
```powershell
# Descargar e instalar rustup desde https://rustup.rs/
# O usar winget en Windows
winget install Rustlang.Rustup
```

### Inicialización del Proyecto
```powershell
cargo init --name api_tareas
cargo add axum tokio mongodb serde chrono bson tower tower-http
cargo add -F tokio/macros,tokio/rt-multi-thread
cargo add -F axum/json
cargo add -F mongodb/tokio-runtime
cargo add -F serde/derive
cargo add -F chrono/serde
```

### Comandos de Desarrollo
- `cargo build` - Compilar
- `cargo run` - Ejecutar la API
- `cargo test` - Ejecutar tests
- `cargo fmt` - Formatear código
- `cargo clippy` - Linter de Rust

## Arquitectura Sugerida

```
src/
├── main.rs           # Punto de entrada, configuración del servidor
├── models/
│   └── tarea.rs      # Definición de Tarea y EstadoTarea
├── handlers/
│   └── tareas.rs     # Controladores CRUD (crear, listar, actualizar, eliminar)
├── db/
│   └── mongo.rs      # Conexión y operaciones MongoDB
└── routes/
    └── mod.rs        # Definición de rutas REST
```

## Endpoints Esperados

- `POST /tareas` - Crear tarea (auto-asignar fecha_creacion)
- `GET /tareas` - Listar todas las tareas
- `GET /tareas/{id}` - Obtener una tarea
- `PUT /tareas/{id}` - Actualizar tarea (cambiar descripcion o estado)
- `DELETE /tareas/{id}` - Eliminar tarea

## Consideraciones de Implementación

- Usar `chrono::Utc::now()` para timestamps automáticos
- Implementar `Serialize` y `Deserialize` de serde en todos los modelos
- Manejar errores con `Result<T, E>` y tipos de error apropiados
- Validar transiciones de estado si es necesario
- Configurar CORS si se planea frontend separado

## Base de Datos

- **Colección**: `tareas`
- **Base de datos**: Crear automáticamente o usar nombre definido (ej: `todo_db`)
- Considerar índices en `estado` para consultas frecuentes
