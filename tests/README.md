# Тесты проекта

## Структура тестов

```
tests/
├── mod.rs                    # Основной модуль тестов
├── unit/                     # Модульные тесты
│   ├── mod.rs               # Модуль модульных тестов
│   ├── client_tests.rs      # Тесты клиента
│   ├── assistant_tests.rs   # Тесты ассистента
│   ├── completion_tests.rs  # Тесты completion
│   └── chat_tests.rs        # Тесты chat сервиса
└── integration/              # Интеграционные тесты
    ├── mod.rs               # Модуль интеграционных тестов
    ├── api_tests.rs         # Тесты API
    └── server_tests.rs      # Тесты сервера
```

## Запуск тестов

### Все тесты
```bash
cargo test --test mod
```

### Только модульные тесты
```bash
cargo test --test mod unit
```

### Только интеграционные тесты
```bash
cargo test --test mod integration
```

### Конкретный тест
```bash
cargo test --test mod test_name
```

## Описание тестов

### Модульные тесты (unit/)

#### client_tests.rs
- `test_client_service_new` - тест создания клиента
- `test_client_service_clone` - тест клонирования клиента
- `test_request_authenticate` - тест аутентификации запросов
- `test_create_success` - тест создания OpenAI клиента
- `test_create_with_invalid_key` - тест с невалидным ключом

#### assistant_tests.rs
- `test_assistant_service_new` - тест создания сервиса ассистента
- `test_wrap_tonic_status_success` - тест обертки статуса
- `test_stream_assistant_creation` - тест создания потокового ассистента
- `test_stream_assistant_with_minimal_params` - тест с минимальными параметрами
- `test_assistant_service_properties` - тест свойств сервиса

#### completion_tests.rs
- `test_completion_service_new` - тест создания сервиса completion
- `test_create_completion_request` - тест создания запроса completion
- `test_create_completion_request_with_stream` - тест с потоком
- `test_completion_service_properties` - тест свойств сервиса
- `test_stream_completion_response_creation` - тест создания потокового ответа
- `test_stream_completion_creation` - тест создания потока completion

#### chat_tests.rs
- `test_get_completion_service` - тест получения сервиса completion
- `test_get_assistant_service` - тест получения сервиса ассистента
- `test_completion_short_message` - тест короткого сообщения для completion
- `test_stream_completion_short_message` - тест короткого сообщения для stream completion
- `test_assistant_short_message` - тест короткого сообщения для assistant
- `test_stream_assistant_short_message` - тест короткого сообщения для stream assistant
- `test_stream_assistant_valid_message` - тест валидного сообщения для stream assistant
- `test_chat_service_creation` - тест создания chat сервиса

### Интеграционные тесты (integration/)

#### api_tests.rs
- `test_chat_service_integration` - интеграционный тест chat сервиса
- `test_service_creation` - тест создания сервисов
- `test_stream_assistant_creation` - тест создания потокового ассистента

#### server_tests.rs
- `test_config_initialization` - тест инициализации конфигурации
- `test_config_default_values` - тест значений по умолчанию
- `test_config_validation` - тест валидации конфигурации
- `test_config_validation_empty_key` - тест валидации с пустым ключом

## Особенности

1. **Моки**: Для тестов используется `ClientService::mock()` который создает тестовый клиент
2. **Переменные окружения**: В тестах устанавливается `OPENAI_API_KEY=test_key`
3. **Потоки**: Для тестов с потоками не используется `unwrap_err()` из-за отсутствия реализации `Debug`
4. **Валидация**: Тесты проверяют валидацию коротких сообщений (< 3 символов)

## Результаты

Все 31 тест проходят успешно, покрывая:
- Создание и работу клиентов
- Создание и работу сервисов
- Валидацию входных данных
- Обработку ошибок
- Потоковые операции
- Конфигурацию приложения 