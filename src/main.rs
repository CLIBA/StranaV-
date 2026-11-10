// A release build on Windows opens no console window, like the original.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

fn main() {
    strana::main();
}
