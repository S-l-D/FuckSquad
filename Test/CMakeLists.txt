add_executable(Test)

target_sources(Test PRIVATE
    # test_main.cpp
    # test_algorithm.cpp
)
target_link_libraries(Test PRIVATE 
    Algorithm 
    DataBase 
    InOut
)

# Автоматическое копирование папки с тестовыми данными (Data) в папку сборки к бинарнику тестов
add_custom_command(
    TARGET Test POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    "${CMAKE_SOURCE_DIR}/Data"
    "$<TARGET_FILE_DIR:TestModule>/Data"
    COMMENT "Копирование тестовых данных в директорию сборки..."
)
