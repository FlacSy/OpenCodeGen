# OpenCodeGen

**OpenCodeGen** — это проект, который автоматически генерирует Python классы на основе спецификаций OpenAPI в формате JSON. Проект использует FastAPI для работы с API, Rust для конвертации и Vue.js для фронтенда.

Вот пример красивых плашек для стека технологий в вашем проекте:


## Стек технологий

### **FastAPI** 
<img src="https://img.shields.io/badge/FastAPI-%231FA2A0.svg?style=for-the-badge&logo=fastapi&logoColor=white" alt="FastAPI" style="vertical-align: middle;"/>
Фреймворк для создания API на Python.

### **Rust (PyO3)**
<img src="https://img.shields.io/badge/Rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" style="vertical-align: middle;"/>
Используется для конвертации OpenAPI в Python объекты через PyO3.

### **Vue.js**
<img src="https://img.shields.io/badge/Vue.js-%234FC08D.svg?style=for-the-badge&logo=vue.js&logoColor=white" alt="Vue.js" style="vertical-align: middle;"/>
Фреймворк для создания фронтенда.

### **Maturin**
<img src="https://img.shields.io/badge/Maturin-%2300A9C4.svg?style=for-the-badge&logo=python&logoColor=white" alt="Maturin" style="vertical-align: middle;"/>
Инструмент для работы с Python-Rust связкой через PyO3.


## Установка и запуск

Для того чтобы запустить проект, выполните следующие шаги:

1. Создайте виртуальное окружение:

    ```bash
    python3 -m venv venv
    source venv/bin/activate  # Для Linux/macOS
    venv\Scripts\activate     # Для Windows
    ```

2. Установите зависимости:

    ```bash
    pip install -r requirements.txt
    ```

3. Установите и соберите Rust пакет с помощью `maturin`:

    - Убедитесь, что у вас установлен Rust и `maturin`.
    - Перейдите в папку `app/backend/code-generator` и выполните команду:

    ```bash
    maturin develop
    ```

4. Запустите сервер:

    ```bash
    uvicorn run:app --reload
    ```


## Журнал изменений

### **0.1.0-beta1** — текущая версия

- **Особенности**:
  - OpenAPI — Python

  - Низкий уровень оптимизации

  - В инпут поле отсутствует проверка валидности JSON



## Планы на будущее

- Поддержка других форматов спецификаций (например, YAML).
- Оптимизация кода и улучшение производительности.
- Добавление поддержки других языков программирования для генерации классов.
- Расширение функционала API.

## Лицензия

Этот проект лицензируется под лицензией FOUL. Подробнее см. в файле [LICENSE](LICENSE).