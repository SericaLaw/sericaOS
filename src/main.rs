#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]


use serica_os::println;
global_asm!(include_str!("boot/entry.asm"));


#[no_mangle]
pub extern "C" fn os_start() -> ! {
    println!(" _____   _____   _____    _   _____       ___   _____   _____");
    println!("/  ___/ | ____| |  _  \\  | | /  ___|     /   | /  _  \\ /  ___/");
    println!("| |___  | |__   | |_| |  | | | |        / /| | | | | | | |___");
    println!("\\___  \\ |  __|  |  _  /  | | | |       / / | | | | | | \\___  \\");
    println!(" ___| | | |___  | | \\ \\  | | | |___   / /  | | | |_| |  ___| |");
    println!("/_____/ |_____| |_|  \\_\\ |_| \\_____| /_/   |_| \\_____/ /_____/\n");
    println!("Welcome to sericaOS v0.1.0");
    loop {}
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
