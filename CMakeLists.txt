cmake_minimum_required(VERSION 3.21)

# Project source files
set(SourceFiles
        src/main.cpp
        src/core/csctrl_minimal.h
        src/logger/logger.cpp
        src/logger/logger.h
)

# Project main settings
project(csctrl LANGUAGES CXX)
set_property(GLOBAL PROPERTY USE_FOLDERS TRUE)
set(CMAKE_CXX_STANDARD          20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS        OFF)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)
option(BUILD_SHARED_LIBS "Build using shared libraries" OFF)

if(MSVC)
    target_compile_options(csctrl PUBLIC "/MP")
endif()

# Create file structure in IDE
foreach(SourceFile IN ITEMS ${SourceFiles})
    get_filename_component(SourcePath ${SourceFile} PATH)
    string(REPLACE ${CMAKE_SOURCE_DIR} "" GroupPath ${SourcePath})
    string(REPLACE "/" "\\" GroupPath ${GroupPath})
    source_group(${GroupPath} FILES ${SourceFile})
endforeach()

# External
add_subdirectory(external/spdlog)
set_target_properties(spdlog PROPERTIES FOLDER external)

option(TRACY_ENABLE "" ON)
option(TRACY_ON_DEMAND "" ON)
add_subdirectory(external/tracy)
set_target_properties(TracyClient PROPERTIES FOLDER external)

# Project target and include directories
add_executable(csctrl ${SourceFiles})
target_include_directories(csctrl PUBLIC src/)
target_link_libraries(csctrl
        spdlog::spdlog
        Tracy::TracyClient
)