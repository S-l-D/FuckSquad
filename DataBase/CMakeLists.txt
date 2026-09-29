# Создаем статическую C++ библиотеку для работы с БД
add_library(DataBase STATIC "")

# Сюда разработчики запишут исходники (например: SqliteWrapper.cpp)
target_sources(DataBase PRIVATE
    # DatabaseStub.cpp
)

# Позволяет писать #include "MyHeader.h" внутри папки и из других модулей
target_include_directories(DataBase PUBLIC .)
