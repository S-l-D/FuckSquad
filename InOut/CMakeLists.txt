# Создаем статическую C++ библиотеку для импорта/экспорта файлов топологии
add_library(InOut STATIC "")
target_sources(InOut
    PRIVATE
        Parser.cpp
    PUBLIC
        FILE_SET HEADERS
        BASE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}"
        FILES
            Parser.h
)

# 2. Экспорт путей поиска заголовков (чтобы Application и Test могли делать #include "parser.h")
target_include_directories(InOut
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
        $<INSTALL_INTERFACE:include>
)

# 3. Линковка зависимостей:
# - DataBase: PUBLIC, потому что тип DataBase используется в сигнатурах методов parser.h
# - nlohmann_json::nlohmann_json: PRIVATE, чтобы не транслировать тяжелые шаблоны JSON 
#   в другие модули (Algorithm, GUI), где json.hpp не нужен напрямую
target_link_libraries(InOut
    PUBLIC
        DataBase
    PRIVATE
        nlohmann_json::nlohmann_json
)