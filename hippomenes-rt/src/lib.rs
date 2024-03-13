#![no_std]
use hippomenes_core::{i0_vec, i1_vec, i2_vec};
#[no_mangle]
pub unsafe fn _setup_interrupts() {
    i0_vec::Bits::write(Interrupt0 as *const fn() as usize);
    i1_vec::Bits::write(Interrupt1 as *const fn() as usize);
    i2_vec::Bits::write(Interrupt2 as *const fn() as usize);
}

extern "C" {
    fn Interrupt0();
    fn Interrupt1();
    fn Interrupt2();
}
