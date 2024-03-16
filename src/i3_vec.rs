pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB03);
    write_csr_as!(0xB03);
    set!(0xB03);
    clear!(0xB03);
}
