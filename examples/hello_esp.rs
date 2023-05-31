#![no_std]
#![no_main]
use core::panic::PanicInfo;
use esp_riscv_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hi!");
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
