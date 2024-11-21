pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB04);
    write_csr_as!(0xB04);
    set!(0xB04);
    clear!(0xB04);
}
