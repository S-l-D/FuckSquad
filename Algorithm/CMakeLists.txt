# Создаем статическую С++ библиотеку модуля
add_library(Algorithm STATIC "")

# Сюда вписывать свои исходные файлы
target_sources(Algorithm PRIVATE
    algoritm.cpp
    # GridModel.cpp
)

# Добавление глобальных имен
target_include_directories(Algorithm PUBLIC .)
target_link_libraries(Algorithm PRIVATE DataBase)