pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB01);
    write_csr_as!(0xB01);
    set!(0xB01);
    clear!(0xB01);
}
