#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
#[macro_use]
pub mod macros {}
pub mod mstatus {
    use core::arch::asm;
    use hippomenes_derive::ImmediateAccess;
    pub struct MIE;
    pub struct Bits;
    #[width(2)]
    pub enum Field {
        Val0 = 0b00,
        Val1 = 0b10,
        Val2 = 0b11,
    }
    impl Field {
        #[inline]
        fn set_field(field: Field) {
            match field {
                Val0 => unsafe { (/*ERROR*/) }
                Val1 => unsafe { (/*ERROR*/) }
                Val2 => unsafe { (/*ERROR*/) }
            }
        }
    }
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
        const _MASK: u8 = ((2 ^ 1) - 1) << 3;
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let val = unsafe { Self::_read() };
            (val & Self::_MASK as usize) >> 3
        }
    }
    impl Field {
        const _OFFSET: u8 = 0;
    }
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
