cmake_minimum_required(VERSION 3.27)
project(MilansCLibrary VERSION 1.0.0.0 LANGUAGES C)

set(CMAKE_C_STANDARD 23)
configure_file(main.h.in ../main.h)
add_executable(cat cat.c)
add_executable(MilansCLibrary main.c)
