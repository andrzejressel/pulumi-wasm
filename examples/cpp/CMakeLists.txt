cmake_minimum_required (VERSION 3.20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project ("PulumiNativeCPP")

include_directories(${CMAKE_SOURCE_DIR}/../../pulumi_native_c)

add_executable (PulumiNativeCPP "main.cpp")

if(WIN32)
  target_link_libraries(PulumiNativeCPP userenv wsock32 ws2_32 ntdll bcrypt)
endif()

set_property(TARGET PulumiNativeCPP PROPERTY CXX_STANDARD 20)

target_link_libraries(PulumiNativeCPP "${CMAKE_SOURCE_DIR}/../../target/debug/pulumi_native_c.lib")