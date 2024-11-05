#![no_main]
#![no_std]
mod lang_items;
mod sbi;
#[macro_use]
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    print!("Hello, World!\n");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

fn printOS() {
    extern  "C" {
        static stext: u8;
        static etext: u8;
        static srodata: u8;
        static edata: u8;
    }
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", s_rodata as usize, e_rodata as usize);
    info!(".data [{:#x}, {:#x})", s_data as usize, e_data as usize);
    info!("load range : [%d, %d] start = %d\n", s, e, start);
}