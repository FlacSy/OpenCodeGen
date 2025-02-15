#!/bin/bash

# Проверка на наличие Python
if ! command -v python3 &> /dev/null
then
    echo "Python не установлен. Пожалуйста, установите Python с https://www.python.org/downloads/"
    exit 1
fi

# Проверка на наличие Rust
if ! command -v rustc &> /dev/null
then
    echo "Rust не установлен. Пожалуйста, установите Rust с https://www.rust-lang.org/"
    exit 1
fi

# Создание виртуального окружения, если оно не существует
if [ ! -d "venv" ]; then
    echo "Создаем виртуальное окружение..."
    python3 -m venv venv
else
    echo "Виртуальное окружение уже существует."
fi

# Активация виртуального окружения (кросс-платформенная поддержка)
if [[ "$OSTYPE" == "linux-gnu"* ]] || [[ "$OSTYPE" == "darwin"* ]]; then
    source venv/bin/activate
elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]]; then
    source venv/Scripts/activate
else
    echo "Неизвестная операционная система. Не удалось активировать виртуальное окружение."
    exit 1
fi

# Установка зависимостей из requirements.txt
if [ -f "requirements.txt" ]; then
    echo "Устанавливаем зависимости из requirements.txt..."
    python -m pip install --upgrade pip
    pip install -r requirements.txt
else
    echo "Файл requirements.txt не найден."
    exit 1
fi

# Проверка на наличие maturin
if ! command -v maturin &> /dev/null
then
    echo "maturin не установлен. Пожалуйста, установите maturin."
    exit 1
fi

# Переход в директорию проекта
cd app/backend/code-generator || { echo "Папка не найдена"; exit 1; }

# Установка и сборка Rust пакета с помощью maturin
echo "Устанавливаем и собираем Rust пакет с помощью maturin..."
maturin develop

# Проверка успешности команды maturin
if [ $? -ne 0 ]; then
    echo "Ошибка при установке и сборке пакета с maturin."
    exit 1
fi

# Запуск сервера
cd "../../../"
echo "Запуск сервера с помощью uvicorn..."
uvicorn run:app 

# Проверка успешности запуска
if [ $? -ne 0 ]; then
    echo "Ошибка при запуске сервера."
    exit 1
fi

echo "Установка завершена и сервер запущен!"
