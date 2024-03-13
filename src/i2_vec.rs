pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB02);
    write_csr_as!(0xB02);
    set!(0xB02);
    clear!(0xB02);
}
