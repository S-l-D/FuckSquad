add_executable(Application)

target_sources(Application
    PRIVATE
        main.cpp
)

set_target_properties(Application PROPERTIES
    OUTPUT_NAME "${PROJECT_NAME}"
)

target_link_libraries(Application
    PRIVATE
        DataBase
        InOut
        #Algorithm
)