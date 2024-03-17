pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB40);
    write_csr_as!(0xB40);
    set!(0xB40);
    clear!(0xB40);
}
