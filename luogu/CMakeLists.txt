cmake_minimum_required(VERSION 3.26)

project(luogu)

file(GLOB SOURCE_FILES P**/*.c P**/*.cpp)

foreach(SOURCE_PATH ${SOURCE_FILES})
    get_filename_component(EXECUTABLE_NAME ${SOURCE_PATH} NAME_WE)
    add_executable(${EXECUTABLE_NAME} ${SOURCE_PATH})
endforeach(SOURCE_PATH ${SOURCE_FILES})
