//! cargo build --target thumbv7em-none-eabihf
#![no_std]
#![no_main]

extern crate alloc;

use adsb_deku::Frame;
use hexlit::hex;
use rsadsb_common::Airplanes;

use core::panic::PanicInfo;

use cortex_m_rt::entry;
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let buffer = hex!("8da7c32758ab75f3291315f10261");
    let _ = Frame::from_reader(&buffer[..]).unwrap();
    let _ = Airplanes::new();

    loop { /* .. */ }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
