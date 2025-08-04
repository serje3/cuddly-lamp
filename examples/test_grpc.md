# Примеры тестирования gRPC API

## 🚀 Быстрый старт

### 1. Установите gRPCui
```powershell
# PowerShell
.\scripts\install_grpcui.ps1

# Или вручную
go install github.com/fullstorydev/grpcui/cmd/grpcui@latest
```

### 2. Запустите сервер
```bash
# Установите API ключ
set OPENAI_API_KEY=your_api_key_here

# Запустите сервер
cargo run
```

### 3. Запустите gRPCui
```powershell
.\scripts\start_grpcui.ps1
```

### 4. Откройте браузер
Перейдите на `http://localhost:8080`

## 📝 Примеры запросов

### Completion API

#### Обычный запрос
```json
{
  "message": "Привет! Как дела?"
}
```

#### Streaming запрос
```json
{
  "message": "Расскажи историю о программировании"
}
```

### Assistant API

#### Обычный запрос с ассистентом
```json
{
  "message": "Объясни концепцию ownership в Rust",
  "assistant_id": "asst_123456789",
  "thread_id": "thread_987654321",
  "instructions": "Будь экспертом по Rust и объясняй простым языком"
}
```

#### Streaming запрос с ассистентом
```json
{
  "message": "Напиши код для простого веб-сервера на Rust",
  "assistant_id": "asst_123456789",
  "thread_id": "thread_987654321",
  "instructions": "Пиши чистый, хорошо документированный код"
}
```

## 🔧 Альтернативные инструменты

### Evans (CLI)
```bash
# Установка
go install github.com/ktr0731/evans@latest

# Интерактивный режим
evans -r repl -p 50051

# Прямой вызов
evans -r call -p 50051 chat.Chat.Completion
```

### BloomRPC (GUI)
1. Скачайте с: https://github.com/bloomrpc/bloomrpc/releases
2. Добавьте сервер: `localhost:50051`
3. Импортируйте proto файлы из `rpc/proto/chat/`

## 🐛 Отладка

### Проверка сервера
```bash
# Проверка порта
netstat -an | findstr 50051

# Логи сервера
RUST_LOG=debug cargo run
```

### Тестирование с curl (если настроен gRPC-Web)
```bash
curl -X POST http://localhost:8080/chat.Chat/Completion \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, world!"}'
```

## 📊 Мониторинг

### Метрики сервера
```bash
# Проверка использования памяти
tasklist | findstr openai-grpc

# Проверка сетевых соединений
netstat -an | findstr 50051
```

### Логи
```bash
# Подробные логи
RUST_LOG=debug cargo run

# Логи только ошибок
RUST_LOG=error cargo run
``` 