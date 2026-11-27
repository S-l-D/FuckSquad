add_executable(TestModule)

target_sources(TestModule
    PRIVATE
        test_parser.cpp
        test_database.cpp
)

# Передаем абсолютный путь к папке Data в C++ код тестов
target_compile_definitions(TestModule
    PRIVATE
        TEST_DATA_DIR="${CMAKE_SOURCE_DIR}/Data"
)

target_link_libraries(TestModule
    PRIVATE
        DataBase
        InOut
        GTest::gtest_main
        nlohmann_json::nlohmann_json
)

include(GoogleTest)
gtest_discover_tests(TestModule)