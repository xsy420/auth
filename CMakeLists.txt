cmake_minimum_required(VERSION 3.14)

file(READ "${CMAKE_SOURCE_DIR}/VERSION" VER_RAW)
string(STRIP ${VER_RAW} VER)

project(
  Auth
  DESCRIPTION "A TOTP Authenticator CLI"
  VERSION ${VER})

include(CheckIncludeFile)
include(GNUInstallDirs)

set(AUTH_VERSION ${VER})
set(PREFIX ${CMAKE_INSTALL_PREFIX})
set(INCLUDEDIR ${CMAKE_INSTALL_INCLUDEDIR})
set(BINDIR ${CMAKE_INSTALL_BINDIR})
configure_file(auth.pc.in auth.pc @ONLY)

set(CMAKE_MESSAGE_LOG_LEVEL "STATUS")
set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_EXECUTABLE_ENABLE_EXPORTS ON)

add_library(auth_compiler_options INTERFACE)
target_compile_options(auth_compiler_options INTERFACE
  -Wall
  -Wextra
  -Wno-unused-parameter
  -Wno-unused-value
  -Wno-missing-field-initializers
  -Wno-narrowing
  -Wno-pointer-arith
  -fmacro-prefix-map=${CMAKE_SOURCE_DIR}/=)

target_compile_definitions(auth_compiler_options INTERFACE
  AUTH_VERSION="${AUTH_VERSION}")

if(CMAKE_BUILD_TYPE MATCHES Debug|DEBUG)
  target_compile_definitions(auth_compiler_options INTERFACE AUTH_DEBUG)
  target_compile_options(auth_compiler_options INTERFACE
    -fsanitize=address,undefined
    -fno-omit-frame-pointer)
  target_link_options(auth_compiler_options INTERFACE
    -fsanitize=address,undefined)
else()
  target_compile_options(auth_compiler_options INTERFACE -O3)
  include(CheckIPOSupported)
  check_ipo_supported(RESULT IPO_SUPPORTED)
  if(IPO_SUPPORTED)
    set(CMAKE_INTERPROCEDURAL_OPTIMIZATION ON)
  endif()
endif()

if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
  target_compile_options(auth_compiler_options INTERFACE -Wno-shadow -Wno-undef)
endif()

find_package(PkgConfig REQUIRED)
find_package(Threads REQUIRED)

set(SQLITE_AMALGAMATION_DIR "${CMAKE_BINARY_DIR}/sqlite_amalgamation")
file(MAKE_DIRECTORY ${SQLITE_AMALGAMATION_DIR})

add_custom_command(
  OUTPUT "${SQLITE_AMALGAMATION_DIR}/sqlite3.c" "${SQLITE_AMALGAMATION_DIR}/sqlite3.h"
  COMMAND ${CMAKE_COMMAND} -E make_directory "${SQLITE_AMALGAMATION_DIR}"
  COMMAND cd "${CMAKE_SOURCE_DIR}/subprojects/sqlite" && ./configure && make sqlite3.c
  COMMAND ${CMAKE_COMMAND} -E copy "${CMAKE_SOURCE_DIR}/subprojects/sqlite/sqlite3.c" "${SQLITE_AMALGAMATION_DIR}/"
  COMMAND ${CMAKE_COMMAND} -E copy "${CMAKE_SOURCE_DIR}/subprojects/sqlite/sqlite3.h" "${SQLITE_AMALGAMATION_DIR}/"
  COMMENT "Generating SQLite amalgamation files"
  VERBATIM
)

add_custom_target(sqlite3_amalgamation DEPENDS
  "${SQLITE_AMALGAMATION_DIR}/sqlite3.c"
  "${SQLITE_AMALGAMATION_DIR}/sqlite3.h")

add_library(sqlite3 STATIC "${SQLITE_AMALGAMATION_DIR}/sqlite3.c")
add_dependencies(sqlite3 sqlite3_amalgamation)
target_compile_options(sqlite3 PRIVATE
  -DSQLITE_ENABLE_COLUMN_METADATA=1
  -DSQLITE_THREADSAFE=1
  -DSQLITE_ENABLE_FTS4=1
  -DSQLITE_ENABLE_FTS5=1
  -DSQLITE_ENABLE_JSON1=1
  -DSQLITE_ENABLE_RTREE=1
  -w
)
set_target_properties(sqlite3 PROPERTIES
  POSITION_INDEPENDENT_CODE ON)
target_include_directories(sqlite3 PUBLIC "${SQLITE_AMALGAMATION_DIR}")

add_subdirectory(subprojects/tomlplusplus)
add_subdirectory(subprojects/catch2)
add_subdirectory(subprojects/nlohmann_json EXCLUDE_FROM_ALL)
pkg_check_modules(
  deps
  REQUIRED
  IMPORTED_TARGET
  openssl)

file(GLOB_RECURSE SRCFILES "src/*.cpp")
add_executable(Auth ${SRCFILES})
target_include_directories(Auth PRIVATE
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${CMAKE_CURRENT_SOURCE_DIR}/src)
target_link_libraries(Auth PRIVATE
  auth_compiler_options
  PkgConfig::deps
  Threads::Threads
  tomlplusplus::tomlplusplus
  nlohmann_json::nlohmann_json
  sqlite3)

set(LIBSRCFILES ${SRCFILES})
list(REMOVE_ITEM LIBSRCFILES "${CMAKE_CURRENT_SOURCE_DIR}/src/Main.cpp")
add_library(AuthLib STATIC ${LIBSRCFILES})
target_include_directories(AuthLib
  PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/src)
target_link_libraries(AuthLib
  PUBLIC
    auth_compiler_options
    sqlite3
  PRIVATE
    PkgConfig::deps
    Threads::Threads
    tomlplusplus::tomlplusplus
    nlohmann_json::nlohmann_json)
set_target_properties(AuthLib PROPERTIES
  POSITION_INDEPENDENT_CODE ON)

enable_testing()
include(CTest)
include(Catch)

configure_file(
  ${CMAKE_CURRENT_SOURCE_DIR}/tests/misc/TestEntries.toml
  ${CMAKE_CURRENT_BINARY_DIR}/tests/misc/TestEntries.toml
  COPYONLY)

file(GLOB_RECURSE TESTFILES "tests/*.cpp")
add_executable(AuthTests ${TESTFILES})
target_link_libraries(AuthTests PRIVATE
  auth_compiler_options
  AuthLib
  Catch2::Catch2WithMain)

catch_discover_tests(AuthTests)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

install(TARGETS Auth)

install(
  CODE "execute_process( \
        COMMAND ${CMAKE_COMMAND} -E create_symlink \
        ${CMAKE_INSTALL_FULL_BINDIR}/Auth \
        \"\$ENV{DESTDIR}${CMAKE_INSTALL_FULL_BINDIR}/auth\" \
        )")

file(GLOB_RECURSE MANPAGES "${CMAKE_CURRENT_SOURCE_DIR}/man/*.1")
install(FILES ${MANPAGES} DESTINATION ${CMAKE_INSTALL_MANDIR}/man1)

install(FILES ${CMAKE_BINARY_DIR}/auth.pc
        DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/pkgconfig)