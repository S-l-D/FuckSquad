# Общие настройки компиляции для C++ бэкенда TetrIC
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Генерация compile_commands.json для IDE и clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Принудительная проверка компилятора — требуем только Clang
if(NOT CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    message(FATAL_ERROR "Проект TetrIC требует компилятор Clang! Текущий компилятор: ${CMAKE_CXX_COMPILER_ID}")
endif()

# Строгие флаги компилятора Clang для контроля качества кода
add_compile_options(
    -Wall          # Базовые предупреждения
    -Wextra        # Дополнительные предупреждения
    -Wpedantic     # Строгое соответствие стандарту ISO C++17
    -Wshadow       # Предупреждать о перекрытии переменных
    -Wconversion   # Предупреждать о неявном приведении типов
)

# Флаги векторизации (использование векторных регистров SIMD / AVX2 / FMA)
set(VECTOR_FLAGS "-O3 -march=native -mavx2 -mfma -fvectorize -fslp-vectorize")

# Режим Debug: с отладочными символами (-g), оптимизацией -O3 и векторными регистрами (по твоему ТЗ)
set(CMAKE_CXX_FLAGS_DEBUG "-g ${VECTOR_FLAGS} -DDEBUG")

# Режим Release: максимальная производительность, векторизация, без отладочных проверок
set(CMAKE_CXX_FLAGS_RELEASE "${VECTOR_FLAGS} -DNDEBUG")

# Создаём симлинк compile_commands.json в корне проекта для анализатора IDE
if(CMAKE_EXPORT_COMPILE_COMMANDS AND NOT EXISTS "${CMAKE_SOURCE_DIR}/compile_commands.json")
    execute_process(
        COMMAND ${CMAKE_COMMAND} -E create_symlink
            "${CMAKE_BINARY_DIR}/compile_commands.json"
            "${CMAKE_SOURCE_DIR}/compile_commands.json"
        ERROR_QUIET
    )
endif()