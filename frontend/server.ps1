# Servidor HTTP simple para el frontend
$port = 8080
$path = "C:\Users\JDARD\OneDrive\Escritorio\api_rust\frontend"

Write-Host "🌐 Servidor HTTP iniciado en http://localhost:$port" -ForegroundColor Green
Write-Host "📂 Sirviendo archivos desde: $path" -ForegroundColor Cyan
Write-Host "Presiona Ctrl+C para detener" -ForegroundColor Yellow
Write-Host ""

# Crear listener HTTP
$listener = New-Object System.Net.HttpListener
$listener.Prefixes.Add("http://localhost:$port/")
$listener.Start()

try {
    while ($listener.IsListening) {
        $context = $listener.GetContext()
        $request = $context.Request
        $response = $context.Response
        
        $file = Join-Path $path "index.html"
        
        if (Test-Path $file) {
            $content = [System.IO.File]::ReadAllBytes($file)
            $response.ContentType = "text/html; charset=utf-8"
            $response.ContentLength64 = $content.Length
            $response.OutputStream.Write($content, 0, $content.Length)
        } else {
            $response.StatusCode = 404
            $buffer = [System.Text.Encoding]::UTF8.GetBytes("404 - Archivo no encontrado")
            $response.OutputStream.Write($buffer, 0, $buffer.Length)
        }
        
        $response.Close()
        Write-Host "$(Get-Date -Format 'HH:mm:ss') - $($request.HttpMethod) $($request.Url.AbsolutePath)" -ForegroundColor Gray
    }
} finally {
    $listener.Stop()
}
