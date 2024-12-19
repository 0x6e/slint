# Copyright © SixtyFPS GmbH <info@slint.dev>
# SPDX-License-Identifier: MIT

cmake_minimum_required(VERSION 3.20.0)

if(NOT BOARD)
    message(FATAL_ERROR "No BOARD option specified")
endif()

set(PRINTERDEMO_UI_SOURCE_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/..)

if(BOARD STREQUAL "native_sim/native/64")
    set(Rust_CARGO_TARGET "x86_64-unknown-linux-gnu")
    set(SLINT_LIBRARY_CARGO_FLAGS "-Zbuild-std=core,alloc")
elseif(BOARD STREQUAL "mimxrt1170_evk@B/mimxrt1176/cm7")
    set(Rust_CARGO_TARGET "thumbv7em-none-eabi")
elseif(BOARD STREQUAL "stm32h747i_disco/stm32h747xx/m7")
    set(Rust_CARGO_TARGET "thumbv7em-none-eabi")

    # The non-mcu printer demo does not fit on this device.
    set(PRINTERDEMO_UI_SOURCE_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../../printerdemo_mcu)
else()
    # See rustc --print target-list for available targets. Match that up with you supported board
    # https://docs.zephyrproject.org/latest/boards
    message(FATAL_ERROR "Unsupported BOARD option specified: ${BOARD}")
endif()

find_package(Zephyr)
project(slint_zephyr_printer_demo_mcu LANGUAGES CXX)

set(SLINT_FEATURE_FREESTANDING ON)
set(SLINT_FEATURE_RENDERER_SOFTWARE ON)
set(DEFAULT_SLINT_EMBED_RESOURCES "embed-for-software-renderer" CACHE STRING "")
# set(CMAKE_BUILD_TYPE Release)
set(BUILD_SHARED_LIBS OFF)

# Work around gcc_s linker issue?
set(Rust_CARGO_TARGET_LINK_NATIVE_LIBS "" CACHE INTERNAL "" FORCE)

# TODO: FetchContent?
add_subdirectory(../../.. slint_build)

# The app target is defined by the call to find_package(Zephyr)
target_sources(app PRIVATE src/slint-zephyr.cpp src/main.cpp)
target_link_libraries(app PRIVATE Slint::Slint)

slint_target_sources(app ${PRINTERDEMO_UI_SOURCE_ROOT}/ui/printerdemo.slint)
