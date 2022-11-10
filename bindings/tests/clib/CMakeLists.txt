cmake_minimum_required(VERSION 3.23)
project(RGMLTest)

message("RGML_PATH = $ENV{RGML_OUTPUT_DIR}")

add_executable(RGMLTest "main.c")
target_link_libraries(RGMLTest PUBLIC rgml gcc_s util rt pthread m dl c)
target_link_directories(RGMLTest PUBLIC $ENV{RGML_OUTPUT_DIR})
target_include_directories(RGMLTest PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../..)