@echo off
echo Installing gRPCui...
echo.

echo Checking if Go is installed...
go version
if %errorlevel% neq 0 (
    echo ERROR: Go is not installed!
    echo Please install Go from: https://golang.org/dl/
    pause
    exit /b 1
)

echo.
echo Installing gRPCui...
go install github.com/fullstorydev/grpcui/cmd/grpcui@latest

echo.
echo Checking installation...
grpcui --version

echo.
echo gRPCui installed successfully!
echo You can now run: scripts/start_grpcui.bat
echo.
pause 