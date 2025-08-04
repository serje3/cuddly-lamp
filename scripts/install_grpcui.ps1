Write-Host "Installing gRPCui..." -ForegroundColor Green
Write-Host ""

# Проверяем установку Go
Write-Host "Checking if Go is installed..." -ForegroundColor Yellow
try {
    $goVersion = go version
    Write-Host "Go found: $goVersion" -ForegroundColor Green
} catch {
    Write-Host "ERROR: Go is not installed!" -ForegroundColor Red
    Write-Host "Please install Go from: https://golang.org/dl/" -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "Installing gRPCui..." -ForegroundColor Yellow
go install github.com/fullstorydev/grpcui/cmd/grpcui@latest

Write-Host ""
Write-Host "Checking installation..." -ForegroundColor Yellow
try {
    $grpcuiVersion = grpcui --version
    Write-Host "gRPCui installed: $grpcuiVersion" -ForegroundColor Green
} catch {
    Write-Host "ERROR: gRPCui installation failed!" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "gRPCui installed successfully!" -ForegroundColor Green
Write-Host "You can now run: .\scripts\start_grpcui.ps1" -ForegroundColor Cyan
Write-Host ""
Read-Host "Press Enter to exit" 