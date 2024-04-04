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

pub struct Pins {
    pub pin0: Pin0,
    pub pin1: Pin1,
    pub pin2: Pin2,
    pub pin3: Pin3,
    pub pin4: Pin4,
    _marker: PhantomData<*const ()>,
}

pub struct Pin0 {
    _marker: PhantomData<*const ()>,
}
pub struct Pin1 {
    _marker: PhantomData<*const ()>,
}
pub struct Pin2 {
    _marker: PhantomData<*const ()>,
}
pub struct Pin3 {
    _marker: PhantomData<*const ()>,
}
pub struct Pin4 {
    _marker: PhantomData<*const ()>,
}
impl GPIO {
    pub fn write(&self, val: usize) {
        unsafe {
            gpio::Bits::write(val);
        }
    }

    pub fn pins(self) -> Pins {
        Pins {
            pin0: Pin0 {
                _marker: PhantomData,
            },
            pin1: Pin1 {
                _marker: PhantomData,
            },
            pin2: Pin2 {
                _marker: PhantomData,
            },
            pin3: Pin3 {
                _marker: PhantomData,
            },
            pin4: Pin4 {
                _marker: PhantomData,
            },
            _marker: PhantomData,
        }
    }
}
pub trait Pin: Sized {
    fn set_high(&self);
    fn set_low(&self);
}
impl Pin for Pin0 {
    fn set_high(&self) {
        gpio::Pin0::set()
    }
    fn set_low(&self) {
        gpio::Pin0::clear()
    }
}
impl Pin for Pin1 {
    fn set_high(&self) {
        gpio::Pin1::set()
    }
    fn set_low(&self) {
        gpio::Pin1::clear()
    }
}
impl Pin for Pin2 {
    fn set_high(&self) {
        gpio::Pin2::set()
    }
    fn set_low(&self) {
        gpio::Pin2::clear()
    }
}
impl Pin for Pin3 {
    fn set_high(&self) {
        gpio::Pin3::set()
    }
    fn set_low(&self) {
        gpio::Pin3::clear()
    }
}
impl Pin for Pin4 {
    fn set_high(&self) {
        gpio::Pin4::set()
    }
    fn set_low(&self) {
        gpio::Pin4::clear()
    }
}
impl Timer {
    pub fn write(&self, val: usize) {
        unsafe {
            timer::Bits::write(val);
        }
    }
    pub fn counter_top(&self) -> CounterTop {
        CounterTop {
            _marker: PhantomData,
        }
    }
}
impl I0Timestamp {
    pub fn read(&self) -> usize {
        unsafe { i0_timestamp::Bits::read() }
    }
}
pub struct CounterTop {
    _marker: PhantomData<*const ()>,
}
impl CounterTop {
    pub fn write(&self, val: usize) {
        unsafe {
            timer::Bits::write(val << 4);
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
