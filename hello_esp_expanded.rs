#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use core::panic::PanicInfo;
use riscv_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
#[export_name = "main"]
pub unsafe fn __risc_v_rt__main() -> ! {
    let channels = {
        use core::mem::MaybeUninit;
        use core::ptr;
        use ::rtt_target::UpChannel;
        use ::rtt_target::DownChannel;
        use ::rtt_target::rtt::*;
        #[repr(C)]
        pub struct RttControlBlock {
            header: RttHeader,
            up_channels: [RttChannel; (1 + 0)],
            down_channels: [RttChannel; (0)],
        }
        #[used]
        #[no_mangle]
        #[export_name = "_SEGGER_RTT"]
        pub static mut CONTROL_BLOCK: MaybeUninit<RttControlBlock> = MaybeUninit::uninit();
        unsafe {
            ptr::write_bytes(CONTROL_BLOCK.as_mut_ptr(), 0, 1);
            let cb = &mut *CONTROL_BLOCK.as_mut_ptr();
            let mut name: *const u8 = core::ptr::null();
            name = "Terminal\u{0}".as_bytes().as_ptr();
            let mut mode = ::rtt_target::ChannelMode::NoBlockSkip;
            mode = ::rtt_target::ChannelMode::NoBlockSkip;
            cb.up_channels[0]
                .init(
                    name,
                    mode,
                    {
                        static mut _RTT_CHANNEL_BUFFER: MaybeUninit<[u8; 1024]> = MaybeUninit::uninit();
                        _RTT_CHANNEL_BUFFER.as_mut_ptr()
                    },
                );
            cb.header.init(cb.up_channels.len(), cb.down_channels.len());
            pub struct Channels {
                up: (UpChannel,),
            }
            Channels {
                up: (UpChannel::new(&mut cb.up_channels[0] as *mut _),),
            }
        }
    };
    ::rtt_target::set_print_channel(channels.up.0);
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
    true
}
