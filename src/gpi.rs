#[allow(unused_imports)]
use core::arch::asm;
use hippomenes_derive::CSRAccess;

// Field definitions

// Input pins
#[derive(CSRAccess)]
#[width = 1]
#[offset = 0]
#[address = 0x1]
pub enum Pin0 {}

#[derive(CSRAccess)]
#[width = 1]
#[offset = 1]
#[address = 0x1]
pub enum Pin1 {}

#[derive(CSRAccess)]
#[width = 1]
#[offset = 2]
#[address = 0x1]
pub enum Pin2 {}

#[derive(CSRAccess)]
#[width = 1]
#[offset = 3]
#[address = 0x1]
pub enum Pin3 {}

// CSR as a whole
pub struct Bits;

impl Bits {
    read_csr_as_usize!(0x001);
    write_csr_as!(0x001);
    set!(0x001);
    clear!(0x001);
}
