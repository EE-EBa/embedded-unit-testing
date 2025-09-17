#![no_std]
#![no_main]
#![allow(dead_code)]

use cortex_m_rt::entry;
use panic_halt as _;/* 这个是必须的 */



#[entry]
fn main() -> ! {
    loop {}
}


