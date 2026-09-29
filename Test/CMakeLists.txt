add_executable(TestModule "")

# Сюда школьники будут добавлять файлы своих юнит-тестов бэкенда
target_sources(TestModule PRIVATE
    # test_main.cpp
    # test_algorithm.cpp
)

# Связываем тесты с проверяемыми С++ библиотеками
target_link_libraries(TestModule PRIVATE 
    Algorithm 
    DataBase 
    InOut
)

# Автоматическое копирование папки с тестовыми данными (Data) в папку сборки к бинарнику тестов
add_custom_command(
    TARGET TestModule POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    "${CMAKE_SOURCE_DIR}/Data"
    "$<TARGET_FILE_DIR:TestModule>/Data"
    COMMENT "Копирование тестовых данных в директорию сборки..."
)
