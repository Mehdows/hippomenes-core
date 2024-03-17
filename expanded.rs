#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
#[macro_use]
pub mod macros {}
pub mod gpio {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[width = 1]
    #[offset = 0]
    #[address = 0x0]
    pub enum Pin0 {}
    impl Pin0 {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0x0, 1<<0") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0x0, 1<<0") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0x0, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 0;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 0
        }
    }
    pub struct Bits;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0x000, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0x000, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0x000, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0x000, {0}", in (reg) bits),
            }
        }
    }
}
pub mod i0_vec {
    pub struct Bits;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB00, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0xB00, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB00, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0xB00, {0}", in (reg) bits),
            }
        }
    }
}
pub mod i1_vec {
    pub struct Bits;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB01, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0xB01, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB01, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0xB01, {0}", in (reg) bits),
            }
        }
    }
}
pub mod i2_vec {
    pub struct Bits;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB02, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0xB02, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB02, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0xB02, {0}", in (reg) bits),
            }
        }
    }
}
pub mod interrupt0 {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[width = 1]
    #[offset = 0]
    #[address = 0xB20]
    pub enum Pending {}
    impl Pending {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0xB20, 1<<0") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0xB20, 1<<0") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB20, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 0;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 0
        }
    }
    #[width = 1]
    #[offset = 1]
    #[address = 0xB20]
    pub enum Enabled {}
    impl Enabled {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0xB20, 1<<1") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0xB20, 1<<1") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB20, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 1;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 1
        }
    }
    #[width = 3]
    #[offset = 2]
    #[address = 0xB20]
    pub enum Priority {}
    impl Priority {
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB20, {0}", in (reg) bits),
            }
        }
        #[inline]
        pub fn set(val: usize) {
            unsafe { Self::_set((val & (2usize.pow(3u8 as u32) - 1)) << 2) };
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB20, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(3u8 as u32)) - 1) << 2;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 2
        }
    }
    pub struct Interrupt0;
    pub struct Bits;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB20, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0xB20, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB20, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0xB20, {0}", in (reg) bits),
            }
        }
    }
    #[allow(non_snake_case)]
    pub mod Timestamp {
        pub struct Bits;
        impl Bits {
            /// Reads the CSR
            #[inline]
            unsafe fn _read() -> usize {
                match () {
                    #[cfg(riscv)]
                    () => {
                        let r: usize;
                        asm!("csrrs {0}, 0xB40, x0", out(reg) r);
                        r
                    }
                }
            }
            /// Reads the CSR
            #[inline]
            pub fn read() -> usize {
                unsafe { Self::_read() }
            }
            /// Writes the CSR
            #[inline]
            #[allow(unused_variables)]
            unsafe fn _write(bits: usize) -> usize {
                match () {
                    #[cfg(riscv)]
                    () => {
                        let v;
                        asm!("csrrw {1}, 0xB40, {0}", in (reg) bits, out(reg) v);
                        v
                    }
                }
            }
            /// Writes the CSR and returns the old value
            #[inline]
            pub fn write(bits: usize) -> usize {
                unsafe { Self::_write(bits) }
            }
            /// Set the CSR
            #[inline]
            #[allow(unused_variables)]
            unsafe fn _set(bits: usize) {
                match () {
                    #[cfg(riscv)]
                    () => asm!("csrrs x0, 0xB40, {0}", in (reg) bits),
                }
            }
            /// Clear the CSR
            #[inline]
            #[allow(unused_variables)]
            unsafe fn _clear(bits: usize) {
                match () {
                    #[cfg(riscv)]
                    () => asm!("csrrc x0, 0xB40, {0}", in (reg) bits),
                }
            }
        }
    }
}
pub mod interrupt1 {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[width = 1]
    #[offset = 0]
    #[address = 0xB21]
    pub enum Pending {}
    impl Pending {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0xB21, 1<<0") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0xB21, 1<<0") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB21, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 0;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 0
        }
    }
    #[width = 1]
    #[offset = 1]
    #[address = 0xB21]
    pub enum Enabled {}
    impl Enabled {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0xB21, 1<<1") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0xB21, 1<<1") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB21, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 1;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 1
        }
    }
    #[width = 3]
    #[offset = 2]
    #[address = 0xB21]
    pub enum Priority {}
    impl Priority {
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB21, {0}", in (reg) bits),
            }
        }
        #[inline]
        pub fn set(val: usize) {
            unsafe { Self::_set((val & (2usize.pow(3u8 as u32) - 1)) << 2) };
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB21, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(3u8 as u32)) - 1) << 2;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 2
        }
    }
    pub struct Bits;
    pub struct Interrupt1;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB21, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0xB21, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB21, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0xB21, {0}", in (reg) bits),
            }
        }
    }
}
pub mod interrupt2 {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[width = 1]
    #[offset = 0]
    #[address = 0xB22]
    pub enum Pending {}
    impl Pending {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0xB22, 1<<0") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0xB22, 1<<0") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB22, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 0;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 0
        }
    }
    #[width = 1]
    #[offset = 1]
    #[address = 0xB22]
    pub enum Enabled {}
    impl Enabled {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0xB22, 1<<1") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0xB22, 1<<1") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB22, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 1;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 1
        }
    }
    #[width = 3]
    #[offset = 2]
    #[address = 0xB22]
    pub enum Priority {}
    impl Priority {
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB22, {0}", in (reg) bits),
            }
        }
        #[inline]
        pub fn set(val: usize) {
            unsafe { Self::_set((val & (2usize.pow(3u8 as u32) - 1)) << 2) };
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB22, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(3u8 as u32)) - 1) << 2;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 2
        }
    }
    pub struct Bits;
    pub struct Interrupt2;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB22, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0xB22, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB22, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0xB22, {0}", in (reg) bits),
            }
        }
    }
}
pub mod interrupt3 {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[width = 1]
    #[offset = 0]
    #[address = 0xB23]
    pub enum Pending {}
    impl Pending {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0xB23, 1<<0") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0xB23, 1<<0") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB23, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 0;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 0
        }
    }
    #[width = 1]
    #[offset = 1]
    #[address = 0xB23]
    pub enum Enabled {}
    impl Enabled {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0xB23, 1<<1") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0xB23, 1<<1") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB23, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 1;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 1
        }
    }
    #[width = 3]
    #[offset = 2]
    #[address = 0xB23]
    pub enum Priority {}
    impl Priority {
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB23, {0}", in (reg) bits),
            }
        }
        #[inline]
        pub fn set(val: usize) {
            unsafe { Self::_set((val & (2usize.pow(3u8 as u32) - 1)) << 2) };
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB23, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(3u8 as u32)) - 1) << 2;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 2
        }
    }
    pub struct Bits;
    pub struct Interrupt3;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0xB23, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0xB23, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0xB23, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0xB23, {0}", in (reg) bits),
            }
        }
    }
}
pub mod mintthresh {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[width = 3]
    #[offset = 0]
    #[address = 0x347]
    #[repr(u8)]
    pub enum Priority {
        Priority0 = 0,
        Priority1 = 1,
        Priority2 = 2,
        Priority3 = 3,
        Priority4 = 4,
        Priority5 = 5,
        Priority6 = 6,
        Priority7 = 7,
    }
    impl Priority {
        #[inline]
        pub fn set_field(field: Priority) {
            match field {
                Priority::Priority0 => unsafe { asm!("csrrsi zero, 0x347, 0<<0") }
                Priority::Priority1 => unsafe { asm!("csrrsi zero, 0x347, 1<<0") }
                Priority::Priority2 => unsafe { asm!("csrrsi zero, 0x347, 2<<0") }
                Priority::Priority3 => unsafe { asm!("csrrsi zero, 0x347, 3<<0") }
                Priority::Priority4 => unsafe { asm!("csrrsi zero, 0x347, 4<<0") }
                Priority::Priority5 => unsafe { asm!("csrrsi zero, 0x347, 5<<0") }
                Priority::Priority6 => unsafe { asm!("csrrsi zero, 0x347, 6<<0") }
                Priority::Priority7 => unsafe { asm!("csrrsi zero, 0x347, 7<<0") }
            }
        }
        pub unsafe fn write_field(field: Priority) {
            match field {
                Priority::Priority0 => unsafe { asm!("csrrsi zero, 0x347, 0<<0") }
                Priority::Priority1 => unsafe { asm!("csrrsi zero, 0x347, 1<<0") }
                Priority::Priority2 => unsafe { asm!("csrrsi zero, 0x347, 2<<0") }
                Priority::Priority3 => unsafe { asm!("csrrsi zero, 0x347, 3<<0") }
                Priority::Priority4 => unsafe { asm!("csrrsi zero, 0x347, 4<<0") }
                Priority::Priority5 => unsafe { asm!("csrrsi zero, 0x347, 5<<0") }
                Priority::Priority6 => unsafe { asm!("csrrsi zero, 0x347, 6<<0") }
                Priority::Priority7 => unsafe { asm!("csrrsi zero, 0x347, 7<<0") }
            }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0x347, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(3u8 as u32)) - 1) << 0u8;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 0u8
        }
    }
    pub struct Bits;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0x347, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0x347, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0x347, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0x347, {0}", in (reg) bits),
            }
        }
    }
}
pub mod mstatus {
    #[allow(unused_imports)]
    use core::arch::asm;
    use hippomenes_derive::CSRAccess;
    #[width = 2]
    #[offset = 0]
    #[address = 0x300]
    pub enum Field {
        Val0 = 0b00,
        Val1 = 0b10,
        Val2 = 0b11,
    }
    impl Field {
        #[inline]
        pub fn set_field(field: Field) {
            match field {
                Field::Val0 => unsafe { asm!("csrrsi zero, 0x300, 0b00<<0") }
                Field::Val1 => unsafe { asm!("csrrsi zero, 0x300, 0b10<<0") }
                Field::Val2 => unsafe { asm!("csrrsi zero, 0x300, 0b11<<0") }
            }
        }
        pub unsafe fn write_field(field: Field) {
            match field {
                Field::Val0 => unsafe { asm!("csrrsi zero, 0x300, 0b00<<0") }
                Field::Val1 => unsafe { asm!("csrrsi zero, 0x300, 0b10<<0") }
                Field::Val2 => unsafe { asm!("csrrsi zero, 0x300, 0b11<<0") }
            }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0x300, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(2u8 as u32)) - 1) << 0u8;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 0u8
        }
    }
    #[width = 1]
    #[offset = 3]
    #[address = 0x300]
    pub enum MIE {}
    impl MIE {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0x300, 1<<3") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0x300, 1<<3") }
        }
        #[inline]
        pub fn clear() {
            unsafe { Self::_clear_imm() }
        }
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0x300, x0", out(reg) r);
                    r
                }
            }
        }
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 3;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 3
        }
    }
    pub struct Bits;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0x300, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0x300, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0x300, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0x300, {0}", in (reg) bits),
            }
        }
    }
}
pub mod timer {
    pub struct Bits;
    impl Bits {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let r: usize;
                    asm!("csrrs {0}, 0x400, x0", out(reg) r);
                    r
                }
            }
        }
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { Self::_read() }
        }
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) -> usize {
            match () {
                #[cfg(riscv)]
                () => {
                    let v;
                    asm!("csrrw {1}, 0x400, {0}", in (reg) bits, out(reg) v);
                    v
                }
            }
        }
        /// Writes the CSR and returns the old value
        #[inline]
        pub fn write(bits: usize) -> usize {
            unsafe { Self::_write(bits) }
        }
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrs x0, 0x400, {0}", in (reg) bits),
            }
        }
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(riscv)]
                () => asm!("csrrc x0, 0x400, {0}", in (reg) bits),
            }
        }
    }
}
pub struct Peripherals {}
impl Peripherals {
    pub fn steal() -> Peripherals {
        Peripherals {}
    }
}
pub use interrupt0::Interrupt0;
pub use interrupt1::Interrupt1;
pub use interrupt2::Interrupt2;
pub use interrupt3::Interrupt3;
pub unsafe trait Interrupt {
    unsafe fn pend_int();
    unsafe fn clear_int();
    unsafe fn enable_int();
    unsafe fn disable_int();
    unsafe fn set_priority(prio: u8);
}
unsafe impl Interrupt for interrupt0::Interrupt0 {
    #[inline(always)]
    unsafe fn pend_int() {
        interrupt0::Pending::set();
    }
    #[inline(always)]
    unsafe fn enable_int() {
        interrupt0::Enabled::set();
    }
    #[inline(always)]
    unsafe fn disable_int() {
        interrupt0::Enabled::clear();
    }
    #[inline(always)]
    unsafe fn set_priority(prio: u8) {
        interrupt0::Priority::set(prio as usize);
    }
    #[inline(always)]
    unsafe fn clear_int() {
        interrupt1::Pending::clear();
    }
}
unsafe impl Interrupt for interrupt1::Interrupt1 {
    #[inline(always)]
    unsafe fn pend_int() {
        interrupt1::Pending::set();
    }
    #[inline(always)]
    unsafe fn enable_int() {
        interrupt1::Enabled::set();
    }
    #[inline(always)]
    unsafe fn disable_int() {
        interrupt1::Enabled::clear();
    }
    #[inline(always)]
    unsafe fn set_priority(prio: u8) {
        interrupt1::Priority::set(prio as usize);
    }
    #[inline(always)]
    unsafe fn clear_int() {
        interrupt1::Pending::clear();
    }
}
unsafe impl Interrupt for interrupt2::Interrupt2 {
    #[inline(always)]
    unsafe fn pend_int() {
        interrupt2::Pending::set();
    }
    #[inline(always)]
    unsafe fn enable_int() {
        interrupt2::Enabled::set();
    }
    #[inline(always)]
    unsafe fn disable_int() {
        interrupt2::Enabled::clear();
    }
    #[inline(always)]
    unsafe fn set_priority(prio: u8) {
        interrupt2::Priority::set(prio as usize);
    }
    #[inline(always)]
    unsafe fn clear_int() {
        interrupt1::Pending::clear();
    }
}
unsafe impl Interrupt for interrupt3::Interrupt3 {
    #[inline(always)]
    unsafe fn pend_int() {
        interrupt3::Pending::set();
    }
    #[inline(always)]
    unsafe fn enable_int() {
        interrupt3::Enabled::set();
    }
    #[inline(always)]
    unsafe fn disable_int() {
        interrupt3::Enabled::clear();
    }
    #[inline(always)]
    unsafe fn set_priority(prio: u8) {
        interrupt3::Priority::set(prio as usize);
    }
    #[inline(always)]
    unsafe fn clear_int() {
        interrupt3::Pending::clear();
    }
}
