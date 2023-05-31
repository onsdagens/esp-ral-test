#![no_std]
#![no_main]
use rtt_target::{rtt_init_print, rprintln};
use core::panic::PanicInfo;
use esp_riscv_rt::{entry};
use riscv::asm;
use esp32c3_ral as ral;
#[entry]
unsafe fn main()->!{
    rtt_init_print!();
   let gpio = ral::gpio::GPIO::instance();
   let iomux = ral::io_mux::IO_MUX::instance();
    ral::modify_reg!(ral::io_mux, iomux, GPIO[7], MCU_SEL: 0b1); //this is mcu_sel i think
    ral::modify_reg!(ral::gpio, gpio, FUNC_OUT_SEL_CFG[7], OUT_SEL:128u32);
    ral::modify_reg!(ral::gpio, gpio, ENABLE, DATA:0b10000000);
    
        ral::modify_reg!(ral::gpio, gpio, OUT, DATA_ORIG:0b10000000);
        rprintln!("Blink on");
        let mut i = 0;
        while i<1000000 {
            asm::nop();
            i+=1;
        }
        ral::modify_reg!(ral::gpio, gpio, OUT, DATA_ORIG:0b00000000);
        rprintln!("Blink off");
        loop{continue;}
}
#[panic_handler]
fn panic(_:&PanicInfo)->!{
    loop{continue;}
}