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

include(FetchContent)
FetchContent_Declare(
  tomlplusplus
  GIT_REPOSITORY https://github.com/marzer/tomlplusplus.git
  GIT_TAG v3.4.0
)
FetchContent_MakeAvailable(tomlplusplus)

find_package(Threads REQUIRED)

pkg_check_modules(
  deps
  REQUIRED
  IMPORTED_TARGET
  openssl)

file(GLOB_RECURSE SRCFILES "src/*.cpp")

add_executable(Auth ${SRCFILES})

target_link_libraries(
  Auth
  PkgConfig::deps
  Threads::Threads
  tomlplusplus::tomlplusplus)

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