pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB00);
    write_csr_as!(0xB00);
    set!(0xB00);
    clear!(0xB00);
}
