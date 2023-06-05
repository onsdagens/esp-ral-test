#![no_std]
#![no_main]
use core::{panic::PanicInfo, arch::global_asm};
use esp32c3_ral as ral;
use riscv_rt::entry;
use esp32c3_barebones_rt;
use riscv::asm;
use core::arch::asm as a;
use rtt_target::{rprintln, rtt_init_print};
#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();
    let gpio = ral::gpio::GPIO::instance();
    let iomux = ral::io_mux::IO_MUX::instance();
    ral::modify_reg!(ral::io_mux, iomux, GPIO[7], MCU_SEL: 0b1); //this is mcu_sel i think
    ral::modify_reg!(ral::gpio, gpio, FUNC_OUT_SEL_CFG[7], OUT_SEL:128u32);
    ral::modify_reg!(ral::gpio, gpio, ENABLE, DATA:0b10000000);
    loop {
        ral::modify_reg!(ral::gpio, gpio, OUT, DATA_ORIG:0b10000000);
        rprintln!("LED on");
        delay(1000000);
        ral::modify_reg!(ral::gpio, gpio, OUT, DATA_ORIG:0b00000000);
        rprintln!("LED off");
        delay(1000000);
        a!("
        csrwi mie,0
        csrwi mip,0
        ");
    }
}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {
        continue;
    }
}

fn delay(ticks:i32){
    let mut i = 0;
    while i<ticks{
        unsafe{asm::nop();}
        i+=1;
    }
}