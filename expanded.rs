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
    pub enum Pin1 {}
    impl Pin1 {
        #[inline]
        unsafe fn _set_imm() {
            unsafe { asm!("csrrsi x0, 0x0, 1<<0u8") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0x0, 1<<0u8") }
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
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 0u8;
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
            unsafe { asm!("csrrsi x0, 0x300, 1<<3u8") }
        }
        #[inline]
        pub fn set() {
            unsafe { Self::_set_imm() }
        }
        #[inline]
        unsafe fn _clear_imm() {
            unsafe { asm!("csrrci x0, 0x300, 1<<3u8") }
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
        const _MASK: u8 = ((2u8.pow(1u8 as u32)) - 1) << 3u8;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 3u8
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
