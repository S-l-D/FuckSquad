# Определяем путь к будущему файлу статической библиотеки Rust

if(CMAKE_BUILD_TYPE MATCHES "Debug")
    set(RUST_BUILD_DIR "debug")
else()
    set(RUST_BUILD_DIR "release")
    set(CARGO_RELEASE_FLAG "--release")
endif()

# Имя файла библиотеки зависит от ОС (на Windows: tetric_gui.lib, на Linux/macOS: libtetric_gui.a)
if(WIN32)
    set(RUST_LIB_FILE "${CMAKE_CURRENT_SOURCE_DIR}/target/${RUST_BUILD_DIR}/tetric_gui.lib")
else()
    set(RUST_LIB_FILE "${CMAKE_CURRENT_SOURCE_DIR}/target/${RUST_BUILD_DIR}/libtetric_gui.a")
endif()

# Команда автоматического вызова Cargo при сборке проекта через CMake
add_custom_command(
    OUTPUT ${RUST_LIB_FILE}
    COMMAND cargo build ${CARGO_RELEASE_FLAG}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Сборка графического модуля GUI на Rust через Cargo..."
)

# Создаем "виртуальную" библиотеку в CMake, привязанную к файлу из Rust
add_library(GUI STATIC IMPORTED GLOBAL)
set_target_properties(GUI PROPERTIES
    IMPORTED_LOCATION ${RUST_LIB_FILE}
)
# Указываем, что библиотека GUI зависит от генерации файла Cargo
add_custom_target(run_cargo DEPENDS ${RUST_LIB_FILE})
add_dependencies(GUI run_cargo)

# Разрешаем подключать эту папку ради заголовочных файлов интерфейса
target_include_directories(GUI INTERFACE .)
