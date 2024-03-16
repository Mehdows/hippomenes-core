#[allow(unused_imports)]
use core::arch::asm;
use hippomenes_derive::CSRAccess;

#[derive(CSRAccess)]
#[width = 3]
#[offset = 0]
#[address = 0x347]
#[repr(u8)]
pub enum Priority {
    Priority0 = 0,
    Priority1 = 1,
    Priority2 = 2,
    Priority3 = 3,
    Priority4 = 4,
    Priority5 = 5,
    Priority6 = 6,
    Priority7 = 7,
}

// The register as a whole
pub struct Bits;

impl Bits {
    read_csr_as_usize!(0x347);
    write_csr_as!(0x347);
    set!(0x347);
    clear!(0x347);
}
