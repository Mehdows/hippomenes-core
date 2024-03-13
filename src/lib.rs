#![no_std]
#[macro_use]
pub mod macros;

pub mod gpio;
pub mod i0_vec;
pub mod i1_vec;
pub mod i2_vec;
pub mod interrupt0;
pub mod interrupt1;
pub mod interrupt2;
pub mod mintthresh;
pub mod mstatus;

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
