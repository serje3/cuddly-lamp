Write-Host "Starting gRPCui..." -ForegroundColor Green
Write-Host ""

# Проверяем, что gRPCui установлен
try {
    $grpcuiVersion = grpcui --version
    Write-Host "gRPCui found: $grpcuiVersion" -ForegroundColor Green
} catch {
    Write-Host "ERROR: gRPCui is not installed!" -ForegroundColor Red
    Write-Host "Please run: .\scripts\install_grpcui.ps1" -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "Make sure your gRPC server is running on localhost:50051" -ForegroundColor Yellow
Write-Host "You can start it with: cargo run" -ForegroundColor Cyan
Write-Host ""

# Проверяем, что сервер запущен
Write-Host "Checking if server is running..." -ForegroundColor Yellow
$portCheck = netstat -an | Select-String "50051"
if ($portCheck) {
    Write-Host "Server appears to be running on port 50051" -ForegroundColor Green
} else {
    Write-Host "WARNING: Server doesn't appear to be running on port 50051" -ForegroundColor Yellow
    Write-Host "Make sure to start the server first!" -ForegroundColor Red
}

Write-Host ""
Write-Host "Starting gRPCui..." -ForegroundColor Green
Write-Host "Web interface will be available at: http://localhost:8080" -ForegroundColor Cyan
Write-Host ""

# Запускаем gRPCui
grpcui -plaintext localhost:50051 