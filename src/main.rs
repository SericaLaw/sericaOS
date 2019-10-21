#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]


use serica_os::println;
use serica_os::{ interrupt, clock };
global_asm!(include_str!("boot/entry.asm"));

const VERSION: &str = "0.1.0";

#[no_mangle]
pub extern "C" fn os_start() -> ! {
    greet();
    interrupt::init();
    clock::init();
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
    println!("Welcome to sericaOS v{}", VERSION);
}