cmake_minimum_required(VERSION 3.0)

project(mathcmd)

add_executable(mathcmd main.cpp)

install(TARGETS mathcmd RUNTIME DESTINATION bin)
