#![no_std]
#![no_main]
use rtt_target::{rtt_init_print, rprintln};
use core::panic::PanicInfo;
//use panic_rtt_target as _;
use esp_riscv_rt::{entry};
// /use gpio;
#[entry]
unsafe fn main()->!{
    rtt_init_print!();
   // let gpio = gpio::take().unwrap();
    rprintln!("Hi!");
    loop{continue;}
}
#[panic_handler]
fn panic(_:&PanicInfo)->!{
    loop{continue;}
}