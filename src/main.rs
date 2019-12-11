#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

use serica_os::{println, uart_println, uart_print};
use serica_os::{interrupt, clock, memory, new_memory, process, consts, device};
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
    // Main should initialize all sub-systems and get
    // ready to start scheduling. The last thing this
    // should do is start the timer.
    greet();
    let mut my_uart = device::uart::Uart::new(0x1000_0000);
    my_uart.init();


    interrupt::init();

//    test_page_table();
    new_memory::init();

    println!("OK");

    process::init();
//    clock::init();
//    process::run();
    uart_println!("Hi");
    let ptr = 0xffff_eff4 as *const i32;
    unsafe {
        println!("0x{:x}", *ptr);
    }

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
//    println!("{}", info);
//    loop {}
    println!("Aborting: ");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    }
    else {
        println!("no information available.");
    }
    abort();
}

#[no_mangle]
extern "C"
fn abort() -> ! {
    // This powers down the hart it's running on until another interrupt.
    loop {
        unsafe {
            asm!("wfi"::::"volatile");
        }
    }
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