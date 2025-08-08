#![no_main]
#![no_std]

use defmt_rtt as _;

use core::ptr::addr_of_mut;
use stm32g4xx_hal as _;
extern crate alloc;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// Define heap memory as a static array
static mut HEAP: [u8; 8192] = [0; 8192]; // 8KB heap

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(addr_of_mut!(HEAP) as *mut u8, 8192);
    }
}

mod json_macro;
use core::panic::PanicInfo;
pub use json_macro::*;

// Standard panic handler required for no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cortex_m::asm::udf()
}

#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
