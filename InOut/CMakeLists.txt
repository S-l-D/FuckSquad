# Создаем статическую C++ библиотеку для импорта/экспорта файлов топологии
add_library(InOut STATIC "")

# Сюда запишут парсеры форматов
target_sources(InOut PRIVATE
    # InOutStub.cpp
)

target_include_directories(InOut PUBLIC .)
