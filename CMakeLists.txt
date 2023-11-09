cmake_minimum_required(VERSION 3.21)

set(SourceFiles
    src/main.cpp
)

project(csctrl LANGUAGES CXX)
set_property(GLOBAL PROPERTY USE_FOLDERS TRUE)
set(CMAKE_CXX_STANDARD          20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS        OFF)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)
# option(BUILD_SHARED_LIBS "Build using shared libraries" ON)

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

add_executable(csctrl ${SourceFiles})
target_include_directories(csctrl PUBLIC src/)
