macro_rules! read_csr {
    ($csr_number:literal) => {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    core::arch::asm!(concat!("csrrs {0}, ", stringify!($csr_number), ", x0"), out(reg) r);
                    r
                }

                #[cfg(not(riscv))]
                () => unimplemented!(),
            }
        }
    };
}

macro_rules! write_csr {
    ($csr_number:literal) => {
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () =>
                    {
                        let v;
                        core::arch::asm!(concat!("csrrw {1}, ", stringify!($csr_number), ", {0}"), in(reg) bits, out(reg) v);
                        v
                    },

                #[cfg(not(riscv))]
                () => unimplemented!(),
            }
        }
    };
}

macro_rules! write_csr_as {
    ($csr_number:literal) => {
        write_csr!($csr_number);

        /// Writes the CSR and returns the old value
        #[inline]
        pub unsafe fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
    };
}

macro_rules! clear {
    ($csr_number:literal) => {
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => core::arch::asm!(concat!("csrrc x0, ", stringify!($csr_number), ", {0}"), in(reg) bits),

                #[cfg(not(riscv))]
                () => unimplemented!(),
            }
        }
    };
}

macro_rules! set {
    ($csr_number:literal) => {
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => core::arch::asm!(concat!("csrrs x0, ", stringify!($csr_number), ", {0}"), in(reg) bits),

                #[cfg(not(riscv))]
                () => unimplemented!(),
            }
        }
    };
}
/*
macro_rules! set_csr {
    ($(#[$attr:meta])*, $set_field:ident, $e:expr) => {
        $(#[$attr])*
        #[inline]
        pub unsafe fn $set_field() {
            _set($e);
        }
    };
}*/
// set via immediate for fields of len 1
macro_rules! set_imm_1 {
    ($csr_number: literal, $offset: expr) => {
        #[inline]
        unsafe fn _set_imm() {
            unsafe {
                core::arch::asm!(concat!(
                    "csrrsi x0, ",
                    stringify!($csr_number),
                    ", 1<<",
                    stringify!($offset)
                ),)
            }
        }
    };
}
macro_rules! clear_imm_1 {
    ($csr_number: literal, $offset: expr) => {
        #[inline]
        unsafe fn _clear_imm() {
            unsafe {
                core::arch::asm!(concat!(
                    "csrrci x0, ",
                    stringify!($csr_number),
                    ", 1<<",
                    stringify!($offset)
                ),)
            }
        }
    };
}
/*
macro_rules! clear_csr {
    ($(#[$attr:meta])*, $clear_field:ident, $e:expr) => {
        $(#[$attr])*
        #[inline]
        pub unsafe fn $clear_field() {
            _clear($e);
        }
    };
}
*/
macro_rules! read_csr_as_usize {
    ($csr_number:literal) => {
        read_csr!($csr_number);

        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
    };
}

macro_rules! field_mask {
    ($width: expr, $offset:expr) => {
        const _MASK: u8 = ((2u8.pow($width as u32)) - 1) << $offset;
    };
}

macro_rules! read_field_as_usize {
    ($csr_number:literal, $width:expr, $offset:expr) => {
        read_csr!($csr_number);
        field_mask!($width, $offset);
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> $offset
        }
    };
}

macro_rules! set_field {
    ($csr_number: literal, 1u8, $offset:expr) => {
        set_imm_1!($csr_number, $offset);

        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
    };
    ($csr_number: literal, $width: expr, $offset:expr) => {
        set!($csr_number);
        #[inline]
        pub fn set(val: usize) {
            unsafe { Self::_set((val & (2usize.pow($width as u32) - 1)) << $offset) };
        }
    };
}

macro_rules! clear_field {
    ($csr_number: literal, 1u8, $offset:expr) => {
        clear_imm_1!($csr_number, $offset);

        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
    };
}
