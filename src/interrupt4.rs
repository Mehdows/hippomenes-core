#[allow(unused_imports)]
use core::arch::asm;
use hippomenes_derive::CSRAccess;
// Field definitions
#[derive(CSRAccess)]
#[width = 1]
#[offset = 0]
#[address = 0xB24]
pub enum Pending {}
#[derive(CSRAccess)]
#[width = 1]
#[offset = 1]
#[address = 0xB24]
pub enum Enabled {}
#[derive(CSRAccess)]
#[width = 3]
#[offset = 2]
#[address = 0xB24]
pub enum Priority {}

pub struct Bits;
// marker
pub struct Interrupt4;
impl Bits {
    read_csr_as_usize!(0xB24);
    write_csr_as!(0xB24);
    set!(0xB24);
    clear!(0xB24);
}
