cmake_minimum_required (VERSION 3.20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project ("PulumiNativeCPP")

include_directories(${CMAKE_SOURCE_DIR}/../../core)

add_executable (PulumiNativeCPP "main.cpp")
set_property(TARGET PulumiNativeCPP PROPERTY CXX_STANDARD 20)

add_library(library SHARED IMPORTED)

if(WIN32)
  target_link_libraries(PulumiNativeCPP userenv wsock32 ws2_32 ntdll bcrypt)
  set_target_properties(library PROPERTIES
    IMPORTED_LOCATION "${CMAKE_SOURCE_DIR}/../../../../../target/debug/pulumi_gestalt_adapter_c.dll"
    IMPORTED_IMPLIB "${CMAKE_SOURCE_DIR}/../../../../../target/debug/pulumi_gestalt_adapter_c.lib"
  )
endif()

if (UNIX AND NOT APPLE)
  set_target_properties(library PROPERTIES
    IMPORTED_LOCATION "${CMAKE_SOURCE_DIR}/../../../../../target/debug/libpulumi_gestalt_adapter_c.so"
    IMPORTED_IMPLIB "${CMAKE_SOURCE_DIR}/../../../../../target/debug/libpulumi_gestalt_adapter_c.a"
  )
endif()

if(APPLE)
  set_target_properties(library PROPERTIES
    IMPORTED_LOCATION "${CMAKE_SOURCE_DIR}/../../../../../target/debug/libpulumi_gestalt_adapter_c.dylib"
    IMPORTED_IMPLIB "${CMAKE_SOURCE_DIR}/../../../../../target/debug/libpulumi_gestalt_adapter_c.a"
  )
endif()

target_link_libraries(PulumiNativeCPP library)

