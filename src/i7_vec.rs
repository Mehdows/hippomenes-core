pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB07);
    write_csr_as!(0xB07);
    set!(0xB07);
    clear!(0xB07);
}
