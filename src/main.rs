#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]


use serica_os::println;
use serica_os::{interrupt, clock, memory, consts};
global_asm!(include_str!("boot/entry.asm"));



fn test_page_table() {
    // test read
    let ptr = 0xc0400000 as *const u32;
    let value = unsafe { ptr.read() };
    println!("addr: {:?}, value: {:#x}", ptr, value);

//    // test write: page fault!
    unsafe {
        (0xc0000000 as *mut u32).write(0);
    }
}

#[no_mangle]
pub extern "C" fn os_start() -> ! {
    greet();
    interrupt::init();
    clock::init();
    memory::init();
    test_page_table();
//    unsafe {
//        asm!("ebreak"::::"volatile");
//    }
    loop {
//
    }
//    panic!("End of rust_main");
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort!");
}

fn greet() {
    println!(" _____   _____   _____    _   _____       ___   _____   _____");
    println!("/  ___/ | ____| |  _  \\  | | /  ___|     /   | /  _  \\ /  ___/");
    println!("| |___  | |__   | |_| |  | | | |        / /| | | | | | | |___");
    println!("\\___  \\ |  __|  |  _  /  | | | |       / / | | | | | | \\___  \\");
    println!(" ___| | | |___  | | \\ \\  | | | |___   / /  | | | |_| |  ___| |");
    println!("/_____/ |_____| |_|  \\_\\ |_| \\_____| /_/   |_| \\_____/ /_____/\n");
    println!("Welcome to sericaOS v{}", consts::VERSION);
}