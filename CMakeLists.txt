cmake_minimum_required(VERSION 3.11)

set(CMAKE_MODULE_PATH ${CMAKE_PROJECT_DIR}/cmake)

project(CouchJunkie C++)

set(SOURCE_FILES
    cpp/src/main.cpp)

add_executable(CouchJunkie ${SOURCE_FILES})
