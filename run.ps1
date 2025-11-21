# Script para ejecutar la API
Write-Host "🔨 Compilando proyecto..." -ForegroundColor Cyan

$cargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"

if (Test-Path $cargoPath) {
    & $cargoPath build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Compilación exitosa" -ForegroundColor Green
        Write-Host "🚀 Iniciando servidor..." -ForegroundColor Cyan
        & ".\target\release\api_tareas.exe"
    }
} else {
    Write-Host "⚠️ Cargo no encontrado. Intentando con el binario existente..." -ForegroundColor Yellow
    & ".\target\debug\api_tareas.exe"
}
