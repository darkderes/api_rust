# Script de gestión Docker para API de Tareas
param(
    [Parameter(Mandatory=$false)]
    [ValidateSet('start', 'stop', 'restart', 'logs', 'clean', 'status', 'help')]
    [string]$Action = 'help'
)

function Show-Help {
    Write-Host "🐳 Gestor Docker - API de Tareas" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Uso: .\docker.ps1 [accion]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Acciones disponibles:" -ForegroundColor Green
    Write-Host "  start    - Construir y ejecutar todos los contenedores"
    Write-Host "  stop     - Detener todos los contenedores"
    Write-Host "  restart  - Reiniciar todos los contenedores"
    Write-Host "  logs     - Ver logs en tiempo real"
    Write-Host "  clean    - Detener y limpiar todo (incluyendo volúmenes)"
    Write-Host "  status   - Ver estado de los contenedores"
    Write-Host "  help     - Mostrar esta ayuda"
    Write-Host ""
}

function Start-Services {
    Write-Host "🚀 Iniciando servicios..." -ForegroundColor Cyan
    Write-Host "⏳ Esto puede tomar 5-10 minutos la primera vez..." -ForegroundColor Yellow
    docker-compose up --build -d
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✅ Servicios iniciados correctamente!" -ForegroundColor Green
        Write-Host ""
        Write-Host "📱 Accede a la aplicación:" -ForegroundColor Cyan
        Write-Host "   Frontend: http://localhost:8080" -ForegroundColor White
        Write-Host "   API:      http://localhost:3000/tareas" -ForegroundColor White
        Write-Host ""
    } else {
        Write-Host "❌ Error al iniciar servicios" -ForegroundColor Red
    }
}

function Stop-Services {
    Write-Host "🛑 Deteniendo servicios..." -ForegroundColor Yellow
    docker-compose down
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Servicios detenidos" -ForegroundColor Green
    } else {
        Write-Host "❌ Error al detener servicios" -ForegroundColor Red
    }
}

function Restart-Services {
    Write-Host "🔄 Reiniciando servicios..." -ForegroundColor Cyan
    docker-compose restart
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Servicios reiniciados" -ForegroundColor Green
    } else {
        Write-Host "❌ Error al reiniciar servicios" -ForegroundColor Red
    }
}

function Show-Logs {
    Write-Host "📋 Mostrando logs (Ctrl+C para salir)..." -ForegroundColor Cyan
    docker-compose logs -f
}

function Clean-All {
    Write-Host "🧹 Limpieza completa..." -ForegroundColor Yellow
    Write-Host "⚠️  Esto eliminará todos los contenedores y datos!" -ForegroundColor Red
    $confirm = Read-Host "¿Estás seguro? (S/N)"
    
    if ($confirm -eq 'S' -or $confirm -eq 's') {
        docker-compose down -v
        Write-Host "✅ Limpieza completada" -ForegroundColor Green
    } else {
        Write-Host "❌ Operación cancelada" -ForegroundColor Yellow
    }
}

function Show-Status {
    Write-Host "📊 Estado de los servicios:" -ForegroundColor Cyan
    Write-Host ""
    docker-compose ps
    Write-Host ""
    
    # Verificar salud de los servicios
    Write-Host "🔍 Verificando conectividad..." -ForegroundColor Cyan
    
    try {
        $response = Invoke-RestMethod -Uri "http://localhost:3000/tareas" -TimeoutSec 2 -ErrorAction Stop
        Write-Host "✅ API: Funcionando correctamente" -ForegroundColor Green
    } catch {
        Write-Host "❌ API: No responde" -ForegroundColor Red
    }
    
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8080" -TimeoutSec 2 -ErrorAction Stop
        Write-Host "✅ Frontend: Funcionando correctamente" -ForegroundColor Green
    } catch {
        Write-Host "❌ Frontend: No responde" -ForegroundColor Red
    }
}

# Verificar que Docker está instalado
try {
    docker --version | Out-Null
} catch {
    Write-Host "❌ Docker no está instalado o no está en el PATH" -ForegroundColor Red
    Write-Host "Instala Docker Desktop desde: https://www.docker.com/products/docker-desktop/" -ForegroundColor Yellow
    exit 1
}

# Verificar que Docker está corriendo
$dockerRunning = docker info 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Docker no está corriendo" -ForegroundColor Red
    Write-Host "Inicia Docker Desktop y vuelve a intentar" -ForegroundColor Yellow
    exit 1
}

# Ejecutar la acción solicitada
switch ($Action) {
    'start'   { Start-Services }
    'stop'    { Stop-Services }
    'restart' { Restart-Services }
    'logs'    { Show-Logs }
    'clean'   { Clean-All }
    'status'  { Show-Status }
    'help'    { Show-Help }
    default   { Show-Help }
}
