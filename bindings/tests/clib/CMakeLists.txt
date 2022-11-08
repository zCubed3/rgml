cmake_minimum_required(VERSION 3.23)
project(PrismMathTest)

message("PRISM_PATH = $ENV{PRISM_OUTPUT_DIR}")

add_executable(PrismMathTest "main.c")
target_link_libraries(PrismMathTest PUBLIC prism_math gcc_s util rt pthread m dl c)
target_link_directories(PrismMathTest PUBLIC $ENV{PRISM_OUTPUT_DIR})
target_include_directories(PrismMathTest PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../..)