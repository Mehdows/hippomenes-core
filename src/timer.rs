pub struct Bits;

impl Bits {
    read_csr_as_usize!(0x400);
    write_csr_as!(0x400);
    set!(0x400);
    clear!(0x400);
}
