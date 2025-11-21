# API REST de Tareas en Rust

API REST para gestión de tareas (TODO list) construida con Axum y MongoDB.

## Requisitos Previos

- Rust (instalación via rustup)
- MongoDB ejecutándose en `mongodb://localhost:27017/`

## Instalación de Rust

```powershell
winget install Rustlang.Rustup
```

O descarga desde https://rustup.rs/

## Instalación del Proyecto

```powershell
# Las dependencias se instalan automáticamente al compilar
cargo build
```

## Ejecutar la API

```powershell
cargo run
```

La API estará disponible en `http://127.0.0.1:3000`

## Endpoints Disponibles

### Crear Tarea
```http
POST /tareas
Content-Type: application/json

{
  "descripcion": "Aprender Rust",
  "estado": "Pendiente"
}
```

### Listar Todas las Tareas
```http
GET /tareas
```

### Obtener una Tarea
```http
GET /tareas/{id}
```

### Actualizar Tarea
```http
PUT /tareas/{id}
Content-Type: application/json

{
  "descripcion": "Aprender más Rust",
  "estado": "Ejecucion"
}
```

### Eliminar Tarea
```http
DELETE /tareas/{id}
```

## Estados de Tarea

- `Pendiente` - Tarea no iniciada
- `Ejecucion` - Tarea en progreso
- `Realizada` - Tarea completada

## Estructura del Proyecto

```
src/
├── main.rs           # Punto de entrada
├── models/
│   └── tarea.rs      # Modelo de datos Tarea
├── handlers/
│   └── tareas.rs     # Lógica de negocio CRUD
├── db/
│   └── mongo.rs      # Conexión MongoDB
└── routes/
    └── mod.rs        # Definición de rutas
```

## Base de Datos

- **Base de datos**: `todo_db`
- **Colección**: `tareas`
- La fecha de creación se asigna automáticamente al crear una tarea
