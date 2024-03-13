#[allow(unused_imports)]
use core::arch::asm;
use hippomenes_derive::CSRAccess;
// Field definitions
#[derive(CSRAccess)]
#[width = 2]
#[offset = 0]
#[address = 0x300]
pub enum Field {
    Val0 = 0b00,
    Val1 = 0b10,
    Val2 = 0b11,
}
#[derive(CSRAccess)]
#[width = 1]
#[offset = 3]
#[address = 0x300]
//#[offset(1)]
pub enum MIE {}

pub struct Bits;

impl Bits {
    read_csr_as_usize!(0x300);
    write_csr_as!(0x300);
    set!(0x300);
    clear!(0x300);
}
//read_csr_as_usize!(0x300);
//write_csr_as!(0x300);
