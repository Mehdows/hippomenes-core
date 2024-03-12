//pub struct Mstatus {}
use core::arch::asm;
use hippomenes_derive::ImmediateAccess;
pub struct MIE;

pub struct Bits;

//pub struct Field;
//const A: u32 = 1;
#[derive(ImmediateAccess)]
#[width(2)]
pub enum Field {
    Val0 = 0b00,
    Val1 = 0b10,
    Val2 = 0b11,
}
/*impl FieldVals {
/*    const _OFFSET: u8 = 0;
    #[inline]
    fn set_field(some_field: FieldVals) {
        match some_field {
            FieldVals::Val0 => unsafe {
                asm!(concat!(
                    "csrrci x0, 0x300, ",
                    concatcp!(FieldVals::Val0),
                    "<<",
                    stringify!(_OFFSET)
                ))
            },
            FieldVals::Val1 => unsafe {
                asm!(concat!(
                    "csrrci x0, 0x300, ",
                    stringify!(FieldVals::Val1),
                    "<<",
                    stringify!(_OFFSET)
                ))
            },
            FieldVals::Val2 => unsafe {
                asm!(concat!(
                    "csrrci x0, 0x300, ",
                    stringify!(FieldVals::Val2),
                    "<<",
                    stringify!(_OFFSET)
                ))
            },
        }
    }*/
}*/

impl MIE {
    set_field!(0x300, 1, 3);
    clear_field!(0x300, 1, 3);
    read_field_as_usize!(0x300, 1, 3);
}

impl Field {
    const _OFFSET: u8 = 0;
}

impl Bits {
    read_csr_as_usize!(0x300);
    write_csr_as!(0x300);
}
//read_csr_as_usize!(0x300);
//write_csr_as!(0x300);
set!(0x300);
clear!(0x300);
