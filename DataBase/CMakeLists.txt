# Создаем статическую C++ библиотеку для работы с БД
add_library(DataBase STATIC "")

target_sources(DataBase
    PRIVATE
        DataBase.cpp
    PUBLIC
        FILE_SET HEADERS
        BASE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}"
        FILES
            DataBase.h
)

target_include_directories(DataBase
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
        $<INSTALL_INTERFACE:include>
)