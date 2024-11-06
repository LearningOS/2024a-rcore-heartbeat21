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
    print_os();
    print!("Hello, World!\n");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn print_os() {
    extern "C" {
        static stext: u8;
        static etext: u8;
        static srodata: u8;
        static erodata: u8;
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        static skernel: u8;
        static ekernel: u8;
    }
    unsafe {
        info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
        info!("load range : [{:#x}, {:#x}] start = {:#x}\n", skernel as usize, ekernel as usize, ekernel as usize);
    }
}
