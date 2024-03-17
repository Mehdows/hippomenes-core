#![no_std]
#[macro_use]
pub mod macros;

pub mod gpio;
pub mod i0_timestamp;
pub mod i0_vec;
pub mod i1_vec;
pub mod i2_vec;
pub mod interrupt0;
pub mod interrupt1;
pub mod interrupt2;
pub mod interrupt3;
pub mod mintthresh;
pub mod mstatus;
pub mod timer;
pub struct Peripherals {
    pub timer: Timer,
    pub gpio: GPIO,
    pub i0_timestamp: I0Timestamp,
}

static mut _TAKEN: bool = false;

impl Peripherals {
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { _TAKEN } {
                None
            } else {
                unsafe {
                    _TAKEN = true;
                }
                Some(Peripherals {
                    timer: Timer {
                        _marker: PhantomData,
                    },
                    gpio: GPIO {
                        _marker: PhantomData,
                    },
                    i0_timestamp: I0Timestamp {
                        _marker: PhantomData,
                    },
                })
            }
        })
    }
    pub unsafe fn steal() -> Peripherals {
        Peripherals {
            timer: Timer {
                _marker: PhantomData,
            },
            gpio: GPIO {
                _marker: PhantomData,
            },
            i0_timestamp: I0Timestamp {
                _marker: PhantomData,
            },
        }
    }
}
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
pub struct I0Timestamp {
    _marker: PhantomData<*const ()>,
}
pub struct Timer {
    _marker: PhantomData<*const ()>,
}
impl GPIO {
    pub fn write(&self, val: usize) {
        unsafe {
            gpio::Bits::write(val);
        }
    }
}

impl Timer {
    pub fn write(&self, val: usize) {
        unsafe {
            timer::Bits::write(val);
        }
    }
}
impl I0Timestamp {
    pub fn read(&self) {
        unsafe {
            i0_timestamp::Bits::read();
        }
    }
}
use core::marker::PhantomData;

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
