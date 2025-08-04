@echo off
echo Starting gRPCui...
echo.
echo Make sure you have installed gRPCui:
echo go install github.com/fullstorydev/grpcui/cmd/grpcui@latest
echo.
echo Server should be running on localhost:50051
echo.
grpcui -plaintext localhost:50051
pause 