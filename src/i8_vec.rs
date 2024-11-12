pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB08);
    write_csr_as!(0xB08);
    set!(0xB08);
    clear!(0xB08);
}
