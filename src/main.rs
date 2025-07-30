#![no_std]
#![no_main]
#![allow(dead_code)]

extern crate alloc;

use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::ptr::replace;
use cortex_m::singleton;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use linked_list_allocator::LockedHeap;
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
use stm32g4xx_hal::pac;



#[entry]
fn main() -> ! {
    // Define heap memory (use a static array for demonstration)
    static mut HEAP: [u8; 1024] = [0; 1024];
    // Initialize the heap allocator
    unsafe { ALLOCATOR.lock().init(HEAP.as_mut_ptr(), HEAP.len()) }

    let rb = unsafe { pac::ADC1::ptr() }; // `pac` 是生成的模块
    rtt_init_print!();

    loop {
        rprintln!("this is infinite loop.");
    }
}
