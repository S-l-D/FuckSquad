cmake_minimum_required(VERSION 3.23)
project(TetrIC VERSION 1.0 LANGUAGES CXX)

option(WITH_TEST "Build test" ON)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Release" CACHE STRING "Choose build type" FORCE)
endif()

# Подключаем общие настройки (Clang, -O3, векторные регистры)
include(Common.cmake)

# Выходные директории
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")

# =============================================================================
# ВНЕШНИЕ ЗАВИСИМОСТИ
# =============================================================================
include(FetchContent)

find_package(nlohmann_json 3.2.0 QUIET)
if(NOT nlohmann_json_FOUND)
    FetchContent_Declare(
        nlohmann_json
        GIT_REPOSITORY https://github.com/nlohmann/json.git
        GIT_TAG v3.11.3
    )
    FetchContent_MakeAvailable(nlohmann_json)
endif()

if(WITH_TEST)
    enable_testing()
    find_package(GTest QUIET)
    if(NOT GTest_FOUND)
        FetchContent_Declare(
            googletest
            GIT_REPOSITORY https://github.com/google/googletest.git
            GIT_TAG v1.14.0
        )
        FetchContent_MakeAvailable(googletest)
    endif()
endif()

# =============================================================================
# ПОДКЛЮЧЕНИЕ МОДУЛЕЙ ПРОЕКТА
# =============================================================================
add_subdirectory(DataBase)
add_subdirectory(InOut)
#add_subdirectory(Algorithm)
#add_subdirectory(GUI)
add_subdirectory(Application)

if(WITH_TEST AND EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/Test/CMakeLists.txt")
    add_subdirectory(Test)
endif()

# =============================================================================
# ТАРГЕТЫ СБОРКИ: build_debug и build_release
# =============================================================================
set(CMAKE_CMD "${CMAKE_COMMAND}")

# 1. Команда: build_debug
add_custom_target(build_debug
    COMMAND ${CMAKE_CMD} -B "${CMAKE_BINARY_DIR}" -G "${CMAKE_GENERATOR}" -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER} -DCMAKE_BUILD_TYPE=Debug
    COMMAND ${CMAKE_CMD} --build "${CMAKE_BINARY_DIR}"
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
    COMMENT "[TetrIC] Сборка проекта в режиме Debug (-g -O3 + SIMD/AVX2)..."
)

# 2. Команда: build_release
add_custom_target(build_release
    COMMAND ${CMAKE_CMD} -B "${CMAKE_BINARY_DIR}" -G "${CMAKE_GENERATOR}" -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER} -DCMAKE_BUILD_TYPE=Release
    COMMAND ${CMAKE_CMD} --build "${CMAKE_BINARY_DIR}"
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
    COMMENT "[TetrIC] Сборка проекта в режиме Release (-O3 + SIMD/AVX2)..."
)

# =============================================================================
# 3. Команда: doxygen (Генерация документации в папку doc/)
# =============================================================================
find_package(Doxygen QUIET)
if(DOXYGEN_FOUND)
    set(DOXYGEN_OUTPUT_DIRECTORY "${CMAKE_SOURCE_DIR}/doc")
    set(DOXYGEN_GENERATE_HTML YES)
    set(DOXYGEN_GENERATE_LATEX NO)
    set(DOXYGEN_EXTRACT_ALL YES)
    set(DOXYGEN_HAVE_DOT YES)
    set(DOXYGEN_RECURSIVE YES)

    doxygen_add_docs(doxygen
        "${CMAKE_SOURCE_DIR}/DataBase"
        "${CMAKE_SOURCE_DIR}/InOut"
        "${CMAKE_SOURCE_DIR}/Algorithm"
        "${CMAKE_SOURCE_DIR}/Application"
        COMMENT "[TetrIC] Генерация документации Doxygen в папку ${CMAKE_SOURCE_DIR}/doc..."
    )
else()
    add_custom_target(doxygen
        COMMAND ${CMAKE_COMMAND} -E echo "Ошибка: Doxygen не установлен! Запустите ./install_deps.sh"
    )
endif()

# =============================================================================
# 4. Команда: lint и format-all (Автовыравнивание кода через clang-format)
# =============================================================================
find_program(CLANG_FORMAT_EXE NAMES clang-format)

file(GLOB_RECURSE ALL_CXX_SOURCE_FILES
    "${CMAKE_SOURCE_DIR}/DataBase/*.cpp"
    "${CMAKE_SOURCE_DIR}/DataBase/*.h"
    "${CMAKE_SOURCE_DIR}/InOut/*.cpp"
    "${CMAKE_SOURCE_DIR}/InOut/*.h"
    "${CMAKE_SOURCE_DIR}/Algorithm/*.cpp"
    "${CMAKE_SOURCE_DIR}/Algorithm/*.h"
    "${CMAKE_SOURCE_DIR}/Application/*.cpp"
    "${CMAKE_SOURCE_DIR}/Application/*.h"
    "${CMAKE_SOURCE_DIR}/Test/*.cpp"
    "${CMAKE_SOURCE_DIR}/Test/*.h"
)

if(CLANG_FORMAT_EXE)
    add_custom_target(lint
        COMMAND ${CLANG_FORMAT_EXE} -i -style=file ${ALL_CXX_SOURCE_FILES}
        WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
        COMMENT "[TetrIC] Автовыравнивание всех исходных файлов (clang-format)..."
    )
    # Алиас format-all для совместимости с Git pre-commit хуком
    add_custom_target(format-all DEPENDS lint)
else()
    add_custom_target(lint
        COMMAND ${CMAKE_COMMAND} -E echo "Ошибка: clang-format не найден! Установите: sudo apt install clang-format"
    )
endif()