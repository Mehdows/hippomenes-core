pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB06);
    write_csr_as!(0xB06);
    set!(0xB06);
    clear!(0xB06);
}
