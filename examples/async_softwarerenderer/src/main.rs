// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

#![no_std]
#![cfg_attr(not(feature = "simulator"), no_main)]
// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

#[mcu_board_support::entry]
fn main() -> ! {
    mcu_board_support::init();

    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();

    panic!("The async software renderer demo should not quit")
}
