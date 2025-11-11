cmake_minimum_required(VERSION 3.16)
project(rmdx)

if(NOT CMAKE_BUILD_TYPE)
  set(default_build_type "Debug")
  message(STATUS "Set the build type to `${default_build_type}` as none was specified.")
  set(CMAKE_BUILD_TYPE ${default_build_type} CACHE STRING "Chooce the build type." FORCE)
  set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS
  "Debug" "Release" "MinSizeRel" "RelWithDebInfo")
endif()
message(STATUS "rmdx Build Type: ${CMAKE_BUILD_TYPE}")

# Set the version for rmdx
set(rmdx_Version_Major 0)
set(rmdx_Version_Minor 1)
set(rmdx_Version_Patch 0)
set(rmdx_Version_Status "-dev")
set(PROJECT_VERSION
  "${rmdx_Version_Major}.${rmdx_Version_Minor}.${rmdx_Version_Patch}${rmdx_Version_Status}"
)
message(STATUS "${PROJECT_NAME} version: ${PROJECT_VERSION}")

add_executable(${PROJECT_NAME} ./example.cpp)

target_link_directories(${PROJECT_NAME} PRIVATE ./target/release)
target_link_libraries(${PROJECT_NAME} PRIVATE mdx.dll.lib)
