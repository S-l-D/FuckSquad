# Общие настройки компиляции для C++ бэкенда TetrIC
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Принудительная проверка компилятора — требуем только Clang
if(NOT CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    message(FATAL_ERROR "Проект TetrIC требует компилятор Clang! Текущий компилятор: ${CMAKE_CXX_COMPILER_ID}")
endif()

# Строгие флаги компилятора Clang для контроля качества кода школьников
add_compile_options(
    -Wall          # Включить базовые предупреждения
    -Wextra        # Включить дополнительные предупреждения
    -Wpedantic     # Строгое соответствие стандарту ISO C++
    -Wshadow       # Предупреждать, если локальная переменная перекрывает внешнюю
    -Wconversion   # Предупреждать о небезопасном неявном приведении типов
)
