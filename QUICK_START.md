# 🚀 Быстрый старт - gRPC тестирование

## Установка и настройка за 5 минут

### 1. Установите Go (если не установлен)
Скачайте с: https://golang.org/dl/

### 2. Установите gRPCui
```powershell
.\scripts\install_grpcui.ps1
```

### 3. Установите переменную окружения
```cmd
set OPENAI_API_KEY=your_openai_api_key_here
```

### 4. Запустите сервер
```cmd
cargo run
```

### 5. Запустите gRPCui (в новом терминале)
```powershell
.\scripts\start_grpcui.ps1
```

### 6. Откройте браузер
Перейдите на `http://localhost:8080`

## 🎯 Готово!

Теперь вы можете:
- ✅ Тестировать все gRPC методы
- ✅ Отправлять запросы через веб-интерфейс
- ✅ Видеть ответы в реальном времени
- ✅ Тестировать streaming функции

## 📝 Быстрые примеры

### Completion
```json
{
  "message": "Привет! Как дела?"
}
```

### Assistant
```json
{
  "message": "Расскажи о Rust",
  "assistant_id": "asst_123",
  "thread_id": "thread_456",
  "instructions": "Будь полезным ассистентом"
}
```

## 🔧 Альтернативы

### Evans (CLI)
```bash
go install github.com/ktr0731/evans@latest
evans -r repl -p 50051
```

### BloomRPC (GUI)
Скачайте с: https://github.com/bloomrpc/bloomrpc/releases

## 📚 Подробная документация

- [Полное руководство по тестированию](docs/GRPC_TESTING.md)
- [Примеры использования](examples/test_grpc.md)
- [Основной README](README.md) 