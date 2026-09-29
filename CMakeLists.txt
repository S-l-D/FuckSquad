cmake_minimum_required(VERSION 3.23)
project(TetrIC LANGUAGES CXX)

option(WITH_TEST "Build test" ON)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Release" CACHE STRING "Choose build type" FORCE)
endif()

# Подключаем общие настройки (Clang, флаги)
include(Common.cmake)

# Подключаем внутренние С++ библиотеки алгоритмов
add_subdirectory(DataBase)
add_subdirectory(InOut)
add_subdirectory(Algorithm)

# Подключаем интерфейс на Rust и финальное приложение на C++
add_subdirectory(GUI)
add_subdirectory(Application)

if(WITH_TEST)
    add_subdirectory(Test)
endif()

set(CMAKE_CMD "${CMAKE_COMMAND}")

# --- РЕЖИМ DEBUG ---

# Собрать в Debug (Автоматически переконфигурирует проект в Debug и компилирует)
add_custom_target(build_debug
    COMMAND ${CMAKE_CMD} -B "${CMAKE_BINARY_DIR}" -DCMAKE_BUILD_TYPE=Debug
    COMMAND ${CMAKE_CMD} --build "${CMAKE_BINARY_DIR}"
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
    COMMENT "[TetrIC] Авто-переключение и сборка проекта в режиме отладки (Debug)..."
)

# Собрать и СРАЗУ ЗАПУСТИТЬ в Debug
add_custom_target(run_debug
    COMMAND ${CMAKE_CMD} --build "${CMAKE_BINARY_DIR}" --target build_debug
    COMMAND $<TARGET_FILE:Application>
    DEPENDS Application
    COMMENT "[TetrIC] Запуск приложения в режиме отладки (Debug)..."
)

# --- РЕЖИМ RELEASE ---

# Собрать в Release (Автоматически переконфигурирует проект в Release с оптимизацией -O3 и компилирует)
add_custom_target(build_release
    COMMAND ${CMAKE_CMD} -B "${CMAKE_BINARY_DIR}" -DCMAKE_BUILD_TYPE=Release
    COMMAND ${CMAKE_CMD} --build "${CMAKE_BINARY_DIR}"
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
    COMMENT "[TetrIC] Авто-переключение и сборка проекта в режиме оптимизации (Release)..."
)

# Собрать и СРАЗУ ЗАПУСТИТЬ в Release
add_custom_target(run_release
    COMMAND ${CMAKE_CMD} --build "${CMAKE_BINARY_DIR}" --target build_release
    COMMAND $<TARGET_FILE:Application>
    DEPENDS Application
    COMMENT "[TetrIC] Запуск приложения в режиме оптимизации (Release)..."
)

# Запуск тестов 
if(WITH_TEST)
    add_custom_target(run_tests
        COMMAND $<TARGET_FILE:TestModule>
        DEPENDS TestModule
        COMMENT "--------------------------------------------------------\n[TetrIC] Запуск автоматических юнит-тестов...\n--------------------------------------------------------"
    )
endif()