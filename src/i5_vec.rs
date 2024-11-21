pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB05);
    write_csr_as!(0xB05);
    set!(0xB05);
    clear!(0xB05);
}
