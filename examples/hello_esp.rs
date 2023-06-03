#![no_std]
#![no_main]
use core::{panic::PanicInfo, arch::global_asm};
use riscv_rt::entry;
//use esp_riscv_rt::entry;
//use esp_backtrace as _;
use rtt_target::{rprintln, rtt_init_print};
#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();
    rprintln!("init");
    loop {
        continue;
    }

}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {
        continue;
    }
}
#[export_name = "_mp_hook"]
pub extern "Rust" fn mp_hook(_: usize) -> bool {
    //loop{continue;}
    true
}
#[export_name = "DefaultHandler"]
fn custom_interrupt_handler(){
    loop{continue;}
}

#[export_name = "ExceptionHandler"]
fn exception_handler(){
   loop{continue;}
}
#[export_name = "_abs_start"]
fn _abs_start(){
    loop{continue;}
}

global_asm!("
.section .init.esp, \"ax\"
.global _start_esp
_start_esp:
    lui ra, %hi(_abs_start)
    jr 48(ra)
");