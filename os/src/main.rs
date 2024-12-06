//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]

use core::arch::global_asm;

#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod task;
mod lang_items;
mod logging;
mod sbi;
mod sync;
mod loader; 
mod config;
mod timer;
pub mod syscall;
pub mod trap;

global_asm!(include_str!("entry.asm"));
// for link the use level program
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
    core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    trap::init();
    loader::load_apps();
    log::info!("time start: {}", timer::get_time());
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    log::info!("time end: {}", timer::get_time());
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}
