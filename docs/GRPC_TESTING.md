# Тестирование gRPC API

## 🛠️ Инструменты для тестирования gRPC

### 1. gRPCui (Рекомендуется)

#### Установка
```bash
# Требуется Go
go install github.com/fullstorydev/grpcui/cmd/grpcui@latest
```

#### Использование
```bash
# Запуск сервера
cargo run

# В другом терминале запуск gRPCui
grpcui -plaintext localhost:50051
```

После запуска откроется веб-интерфейс на `http://localhost:8080`

### 2. Evans (CLI инструмент)

#### Установка
```bash
go install github.com/ktr0731/evans@latest
```

#### Использование
```bash
# Интерактивный режим
evans -r repl -p 50051

# Прямой вызов
evans -r call -p 50051 chat.Chat.Completion
```

### 3. BloomRPC (GUI приложение)

1. Скачайте с: https://github.com/bloomrpc/bloomrpc/releases
2. Установите приложение
3. Добавьте сервер: `localhost:50051`
4. Импортируйте proto файлы

## 🚀 Быстрый старт

### Шаг 1: Установите gRPCui
```bash
go install github.com/fullstorydev/grpcui/cmd/grpcui@latest
```

### Шаг 2: Запустите сервер
```bash
# Установите переменную окружения
set OPENAI_API_KEY=your_api_key_here

# Запустите сервер
cargo run
```

### Шаг 3: Запустите gRPCui
```bash
grpcui -plaintext localhost:50051
```

### Шаг 4: Откройте браузер
Перейдите на `http://localhost:8080`

## 📋 Доступные методы

### Completion
- **Completion** - обычный запрос
- **StreamCompletion** - streaming запрос

### Assistant
- **Assistant** - обычный запрос с ассистентом
- **StreamAssistant** - streaming запрос с ассистентом

## 📝 Примеры запросов

### Completion
```json
{
  "message": "Привет, как дела?"
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

## 🔧 Настройка для разработки

### Переменные окружения
```bash
OPENAI_API_KEY=your_openai_api_key
BIND_ADDR=127.0.0.1:50051
```

### Запуск в режиме разработки
```bash
# Терминал 1: Сервер
cargo run

# Терминал 2: gRPCui
grpcui -plaintext localhost:50051
```

## 🐛 Отладка

### Проверка подключения
```bash
# Проверка порта
netstat -an | findstr 50051

# Тест с curl (если настроен gRPC-Web)
curl -X POST http://localhost:8080/chat.Chat/Completion
```

### Логи сервера
```bash
# Запуск с подробными логами
RUST_LOG=debug cargo run
```

## 📚 Полезные ссылки

- [gRPCui GitHub](https://github.com/fullstorydev/grpcui)
- [Evans GitHub](https://github.com/ktr0731/evans)
- [BloomRPC GitHub](https://github.com/bloomrpc/bloomrpc)
- [gRPC Web DevTools](https://github.com/grpc/grpc-web) 