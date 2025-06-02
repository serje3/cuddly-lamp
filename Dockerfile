FROM rust:1.87

RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /usr/src/app

# Копируем весь проект
COPY . .

# Собираем в debug-режиме (по умолчанию)
RUN cargo build

# Запускаем проект
CMD ["cargo", "run"]
