# Создаем статическую С++ библиотеку модуля
add_library(Algorithm STATIC "")

# Сюда вписывать свои исходные файлы
target_sources(Algorithm PRIVATE
    # TetrisPlacer.cpp
    # GridModel.cpp
)

# Добавление глобальных имен
target_include_directories(Algorithm PUBLIC .)
