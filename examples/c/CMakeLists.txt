cmake_minimum_required (VERSION 3.20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project ("PulumiNativeC")

include_directories(${CMAKE_SOURCE_DIR}/../../crates/c_ffi)

add_executable (PulumiNativeC "main.c")
set_property(TARGET PulumiNativeC PROPERTY C_STANDARD 23)
set_property(TARGET PulumiNativeC PROPERTY COMPILE_WARNING_AS_ERROR ON)

add_library(library SHARED IMPORTED)

if(WIN32)
  target_link_libraries(PulumiNativeC userenv wsock32 ws2_32 ntdll bcrypt)
  set_target_properties(library PROPERTIES
    IMPORTED_LOCATION "${CMAKE_SOURCE_DIR}/../../target/debug/pulumi_native_c.dll"
    IMPORTED_IMPLIB "${CMAKE_SOURCE_DIR}/../../target/debug/pulumi_native_c.lib"
  )
endif()

if (UNIX AND NOT APPLE)
  set_target_properties(library PROPERTIES
    IMPORTED_LOCATION "${CMAKE_SOURCE_DIR}/../../target/debug/libpulumi_native_c.so"
    IMPORTED_IMPLIB "${CMAKE_SOURCE_DIR}/../../target/debug/libpulumi_native_c.a"
  )
endif()

if(APPLE)
  set_target_properties(library PROPERTIES
    IMPORTED_LOCATION "${CMAKE_SOURCE_DIR}/../../target/debug/libpulumi_native_c.dylib"
    IMPORTED_IMPLIB "${CMAKE_SOURCE_DIR}/../../target/debug/libpulumi_native_c.a"
  )
endif()

target_link_libraries(PulumiNativeC library)