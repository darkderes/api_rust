# 🐳 Guía de Implementación con Docker

Esta guía te ayudará a ejecutar la aplicación de Tareas usando Docker en tu máquina local.

## 📋 Requisitos Previos

1. **Docker Desktop** instalado y ejecutándose
   - Descargar desde: https://www.docker.com/products/docker-desktop/
   - Asegúrate de que Docker Desktop esté corriendo

2. **Verificar instalación**
   ```powershell
   docker --version
   docker-compose --version
   ```

## 🚀 Implementación Local (Primera vez)

### Paso 1: Detener servicios locales (si los tienes corriendo)

Si tienes MongoDB o la API corriendo localmente, detenlos primero:

```powershell
# Detener procesos de Rust
Get-Process | Where-Object {$_.ProcessName -eq "api_tareas"} | Stop-Process -Force

# Detener MongoDB local (si está como servicio)
Stop-Service MongoDB -ErrorAction SilentlyContinue
```

### Paso 2: Construir y ejecutar todos los contenedores

Abre PowerShell en la carpeta del proyecto y ejecuta:

```powershell
cd C:\Users\JDARD\OneDrive\Escritorio\api_rust

# Construir y levantar todos los servicios
docker-compose up --build
```

**Nota:** La primera vez tomará varios minutos (5-10 min) porque:
- Descarga las imágenes base de Docker
- Compila el proyecto Rust en modo release
- Configura MongoDB y Nginx

### Paso 3: Verificar que todo está corriendo

En otra terminal PowerShell, verifica los contenedores:

```powershell
docker-compose ps
```

Deberías ver 3 servicios corriendo:
- `tareas_mongodb` (puerto 27017)
- `tareas_api` (puerto 3000)
- `tareas_frontend` (puerto 8080)

### Paso 4: Acceder a la aplicación

Abre tu navegador y ve a:
- **Frontend**: http://localhost:8080
- **API directa**: http://localhost:3000/tareas

## 🔄 Comandos Útiles

### Ejecutar en segundo plano (detached mode)
```powershell
docker-compose up -d
```

### Ver logs en tiempo real
```powershell
# Todos los servicios
docker-compose logs -f

# Solo la API
docker-compose logs -f api

# Solo el frontend
docker-compose logs -f frontend

# Solo MongoDB
docker-compose logs -f mongodb
```

### Detener todos los servicios
```powershell
docker-compose down
```

### Detener y eliminar volúmenes (limpieza completa)
```powershell
docker-compose down -v
```

### Reiniciar un servicio específico
```powershell
# Reiniciar solo la API
docker-compose restart api

# Reiniciar solo el frontend
docker-compose restart frontend
```

### Reconstruir después de cambios en el código
```powershell
# Reconstruir y reiniciar solo la API
docker-compose up -d --build api

# Reconstruir todo
docker-compose up -d --build
```

### Ver el estado de los contenedores
```powershell
docker-compose ps
```

### Acceder al shell de un contenedor
```powershell
# MongoDB
docker exec -it tareas_mongodb mongosh

# API (shell del contenedor)
docker exec -it tareas_api /bin/bash

# Frontend (Nginx)
docker exec -it tareas_frontend /bin/sh
```

## 🔍 Verificación de Salud

### Verificar MongoDB
```powershell
docker exec -it tareas_mongodb mongosh --eval "db.adminCommand('ping')"
```

### Verificar API
```powershell
Invoke-RestMethod -Uri "http://localhost:3000/tareas"
```

### Verificar Frontend
Abre http://localhost:8080 en el navegador

## 🐛 Solución de Problemas

### Error: Puerto ya en uso
Si ves un error como "port is already allocated":

```powershell
# Ver qué proceso está usando el puerto
Get-NetTCPConnection -LocalPort 3000 | Select-Object OwningProcess | Get-Process

# Cambiar el puerto en docker-compose.yml
# Por ejemplo, cambiar "3000:3000" a "3001:3000"
```

### La API no se conecta a MongoDB
```powershell
# Ver logs de la API
docker-compose logs api

# Verificar que MongoDB esté healthy
docker-compose ps
```

### Reconstruir desde cero
```powershell
# Detener todo y limpiar
docker-compose down -v
docker system prune -a --volumes

# Volver a construir
docker-compose up --build
```

## 📊 Arquitectura de Contenedores

```
┌─────────────────────────────────────────┐
│         Docker Network (Bridge)         │
│                                         │
│  ┌──────────────┐  ┌──────────────┐   │
│  │   Frontend   │  │      API     │   │
│  │   (Nginx)    │──│    (Rust)    │   │
│  │   Port 8080  │  │   Port 3000  │   │
│  └──────────────┘  └───────┬──────┘   │
│                            │           │
│                     ┌──────▼──────┐    │
│                     │   MongoDB   │    │
│                     │  Port 27017 │    │
│                     └─────────────┘    │
│                            │           │
└────────────────────────────┼───────────┘
                             │
                      ┌──────▼──────┐
                      │   Volume    │
                      │ mongodb_data│
                      └─────────────┘
```

## 📦 Persistencia de Datos

Los datos de MongoDB se almacenan en un volumen Docker llamado `mongodb_data`.
- Los datos persisten incluso si detienes los contenedores
- Solo se eliminan si ejecutas `docker-compose down -v`

## 🚀 Próximos Pasos (Producción)

Para desplegar en producción, considera:
1. Usar variables de entorno para configuración sensible
2. Configurar HTTPS con certificados SSL
3. Implementar autenticación en MongoDB
4. Usar registros de contenedores (Docker Hub, AWS ECR, etc.)
5. Desplegar en plataformas cloud (AWS ECS, Google Cloud Run, Azure Container Instances)

## 📝 Notas Importantes

- La primera compilación puede tardar 5-10 minutos
- El modo release de Rust optimiza al máximo el rendimiento
- Los datos de MongoDB persisten entre reinicios
- Puedes acceder directamente a la API en http://localhost:3000
- El frontend está optimizado con Nginx para producción
