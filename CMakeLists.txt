cmake_minimum_required(VERSION 3.28)
project(laborsynth C)

set(CMAKE_C_STANDARD 11)

include_directories(.)

add_executable(laborsynth
        lib.c
        lib.h
        main.c)
