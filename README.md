# OpenAI gRPC Server

Сервер gRPC для работы с OpenAI API, предоставляющий интерфейсы для completion и assistant функций.

## 🚀 Быстрый старт

### Установка зависимостей
```bash
# Установите Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Клонируйте репозиторий
git clone <repository-url>
cd cuddly-lamp

# Установите зависимости
cargo build
```

### Настройка
```bash
# Установите переменную окружения
set OPENAI_API_KEY=your_openai_api_key_here
```

### Запуск сервера
```bash
cargo run
```

Сервер запустится на `127.0.0.1:50051`

## 🛠️ Тестирование API

### Установка gRPCui (рекомендуется)
```powershell
# PowerShell
.\scripts\install_grpcui.ps1

# Или вручную
go install github.com/fullstorydev/grpcui/cmd/grpcui@latest
```

### Запуск тестового интерфейса
```powershell
# Запустите сервер в одном терминале
cargo run

# Запустите gRPCui в другом терминале
.\scripts\start_grpcui.ps1
```

Откройте браузер на `http://localhost:8080`

## 📋 API Методы

### Completion
- `Completion` - обычный запрос к чат-модели
- `StreamCompletion` - streaming запрос

### Assistant
- `Assistant` - запрос к ассистенту с поддержкой потоков
- `StreamAssistant` - streaming запрос к ассистенту

## 📝 Примеры запросов

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

## 🧪 Запуск тестов

```bash
# Все тесты
cargo test

# Конкретный тест
cargo test test_completion

# Тесты с выводом
cargo test -- --nocapture
```

## 📚 Документация

- [Тестирование gRPC API](docs/GRPC_TESTING.md)
- [Примеры использования](examples/test_grpc.md)

## 🔧 Разработка

### Структура проекта
```
src/
├── main.rs              # Точка входа
├── client.rs            # Клиент для OpenAI API
├── services/            # Бизнес-логика
│   ├── assistant.rs     # Сервис ассистентов
│   ├── completion.rs    # Сервис completion
│   └── base.rs          # Базовые трейты
├── svc/                 # gRPC сервисы
│   └── chat.rs          # Chat сервис
└── config/              # Конфигурация
    ├── settings.rs      # Настройки приложения
    ├── env.rs           # Работа с переменными окружения
    └── errors.rs        # Ошибки конфигурации
```

### Переменные окружения
```bash
OPENAI_API_KEY=your_openai_api_key    # Обязательно
BIND_ADDR=127.0.0.1:50051            # Опционально (по умолчанию)
```

## 🐛 Отладка

### Логи
```bash
# Подробные логи
RUST_LOG=debug cargo run

# Логи только ошибок
RUST_LOG=error cargo run
```

### Проверка сервера
```bash
# Проверка порта
netstat -an | findstr 50051

# Проверка процессов
tasklist | findstr openai-grpc
```

## 📦 Альтернативные инструменты тестирования

### Evans (CLI)
```bash
go install github.com/ktr0731/evans@latest
evans -r repl -p 50051
```

### BloomRPC (GUI)
Скачайте с: https://github.com/bloomrpc/bloomrpc/releases

## 📄 Лицензия

MIT License - см. [LICENSE.md](LICENSE.md) 