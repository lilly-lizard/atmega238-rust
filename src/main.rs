#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

use ruduino::cores::atmegs328p as avr_core;
use ruduino::Register;

#[no_mangle]
pub extern "C" fn main() {
    // set all PORTB pins up as output
    DDRB::set_mask_raw(0xFFu8);

    loop {
        // set all pins on PORTB to high
        PORTB::set_mask_raw(0xFF);

        small_delay();

        // set all pins on PORTB to low
        PORTB::unset_mask_raw(0xFF);

        small_delay();
    }
}

fn small_delay() {
    for _ in 0..4000 {
        unsafe { llvm_asm!("" :::: "volatile") }
    }
}
