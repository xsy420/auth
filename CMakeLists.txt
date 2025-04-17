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

find_package(PkgConfig REQUIRED)

if(CMAKE_BUILD_TYPE MATCHES Debug OR CMAKE_BUILD_TYPE MATCHES DEBUG)
  add_compile_definitions(AUTH_DEBUG)
else()
  add_compile_options(-O3)
endif()

add_compile_definitions(AUTH_VERSION="${AUTH_VERSION}")

include_directories(. "include/" "src/")

set(CMAKE_CXX_STANDARD 26)
set(CXX_STANDARD_REQUIRED ON)
add_compile_options(
  -Wall
  -Wextra
  -Wno-unused-parameter
  -Wno-unused-value
  -Wno-missing-field-initializers
  -Wno-narrowing
  -Wno-pointer-arith
  -fmacro-prefix-map=${CMAKE_SOURCE_DIR}/=)

set(CMAKE_EXECUTABLE_ENABLE_EXPORTS TRUE)
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)

if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wno-everything -Wno-shadow -Wno-undef")
endif()
add_subdirectory(subprojects/tomlplusplus)
add_subdirectory(subprojects/catch2)

if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
  string(REGEX REPLACE "-Wno-everything" "" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
endif()

set(SQLITE_AMALGAMATION_DIR "${CMAKE_BINARY_DIR}/sqlite_amalgamation")
file(MAKE_DIRECTORY ${SQLITE_AMALGAMATION_DIR})

add_custom_command(
  OUTPUT "${SQLITE_AMALGAMATION_DIR}/sqlite3.c" "${SQLITE_AMALGAMATION_DIR}/sqlite3.h"
  COMMAND mkdir -p "${SQLITE_AMALGAMATION_DIR}"
  COMMAND cd "${CMAKE_SOURCE_DIR}/subprojects/sqlite" && ./configure && make sqlite3.c
  COMMAND cp "${CMAKE_SOURCE_DIR}/subprojects/sqlite/sqlite3.c" "${SQLITE_AMALGAMATION_DIR}/"
  COMMAND cp "${CMAKE_SOURCE_DIR}/subprojects/sqlite/sqlite3.h" "${SQLITE_AMALGAMATION_DIR}/"
  COMMENT "Generating SQLite amalgamation files"
  VERBATIM
)

add_custom_target(sqlite3_amalgamation DEPENDS "${SQLITE_AMALGAMATION_DIR}/sqlite3.c" "${SQLITE_AMALGAMATION_DIR}/sqlite3.h")

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
set_target_properties(sqlite3 PROPERTIES POSITION_INDEPENDENT_CODE ON)

set(SQLITE3_INCLUDE_DIR "${SQLITE_AMALGAMATION_DIR}")

include_directories(${CMAKE_SOURCE_DIR}/subprojects/nlohmann_json/include ${SQLITE3_INCLUDE_DIR})

find_package(Threads REQUIRED)

pkg_check_modules(
  deps
  REQUIRED
  IMPORTED_TARGET
  openssl)

file(GLOB_RECURSE SRCFILES "src/*.cpp")

add_executable(Auth ${SRCFILES})
add_dependencies(Auth sqlite3)

target_include_directories(Auth PRIVATE ${SQLITE3_INCLUDE_DIR})
target_link_libraries(
  Auth
  PkgConfig::deps
  Threads::Threads
  tomlplusplus::tomlplusplus
  sqlite3)

set(LIBSRCFILES ${SRCFILES})
list(REMOVE_ITEM LIBSRCFILES "${CMAKE_CURRENT_SOURCE_DIR}/src/Main.cpp")
add_library(AuthLib STATIC ${LIBSRCFILES})
add_dependencies(AuthLib sqlite3)

target_include_directories(AuthLib PRIVATE ${SQLITE3_INCLUDE_DIR})
target_link_libraries(
  AuthLib
  PkgConfig::deps
  Threads::Threads
  tomlplusplus::tomlplusplus
  sqlite3)

enable_testing()
include(CTest)
include(Catch)

configure_file(tests/misc/TestEntries.toml tests/misc/TestEntries.toml COPYONLY)

file(GLOB_RECURSE TESTFILES "tests/*.cpp")
add_executable(AuthTests ${TESTFILES})
add_dependencies(AuthTests sqlite3)

target_link_libraries(
  AuthTests
  AuthLib
  Catch2::Catch2WithMain)

add_custom_target(run_tests COMMAND AuthTests)

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

file(GLOB_RECURSE MANPAGES "man/*.1")
install(FILES ${MANPAGES} DESTINATION ${CMAKE_INSTALL_MANDIR}/man1)

install(FILES ${CMAKE_BINARY_DIR}/auth.pc
        DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/pkgconfig)