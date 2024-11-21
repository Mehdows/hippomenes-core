#![no_std]
#[macro_use]
pub mod macros;

pub mod gpi;
pub mod gpo;
pub mod i0_timestamp;
pub mod i0_vec;
pub mod i1_vec;
pub mod i2_vec;
pub mod i3_vec;
pub mod i4_vec;
pub mod i5_vec;
pub mod i6_vec;
pub mod i7_vec;
pub mod i8_vec;
pub mod interrupt0;
pub mod interrupt1;
pub mod interrupt2;
pub mod interrupt3;
pub mod interrupt4;
pub mod interrupt5;
pub mod interrupt6;
pub mod interrupt7;
pub mod mintthresh;
pub mod mpu;
pub mod mstatus;
pub mod timer;
pub mod uart;
pub struct Peripherals {
    pub timer: Timer,
    pub gpi: GPI,
    pub gpo: GPO,
    pub uart: UART,
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
                    gpi: GPI {
                        _marker: PhantomData,
                    },
                    gpo: GPO {
                        _marker: PhantomData,
                    },
                    uart: UART {
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
            gpi: GPI {
                _marker: PhantomData,
            },
            gpo: GPO {
                _marker: PhantomData,
            },
            uart: UART {
                _marker: PhantomData,
            },
            i0_timestamp: I0Timestamp {
                _marker: PhantomData,
            },
        }
    }
}

pub struct GPI {
    _marker: PhantomData<*const ()>,
}

pub struct GPO {
    _marker: PhantomData<*const ()>,
}

pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
pub struct I0Timestamp {
    _marker: PhantomData<*const ()>,
}
pub struct Timer {
    _marker: PhantomData<*const ()>,
}

pub struct MPU {
    pub mpu_config: MPUConfig,
    pub interrupt_1_config: Interrupt1Config,
    
    _marker: PhantomData<*const ()>,
}

pub struct MPUConfig {
    _marker: PhantomData<*const ()>,
}
pub struct Interrupt1Config {
    _marker: PhantomData<*const ()>,
}

impl MPUConfig {
    //pub fn set_permissions(&mut self, )
}
impl Interrupt1Config {
    //pub fn set_permissions(&mut self, )
}

pub struct PinOut {
    pub pout0: Pout0, // Output pins
    pub pout1: Pout1,
    pub pout2: Pout2,
    pub pout3: Pout3,
    pub pout4: Pout4,

    _marker: PhantomData<*const ()>,
}

pub struct PinIn {
    pub pin0: Pin0, // Output pins
    pub pin1: Pin1,
    pub pin2: Pin2,
    pub pin3: Pin3,

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

impl GPI {
    pub fn read(&self) -> usize {
        unsafe { gpi::Bits::read() }
    }

    pub fn split(self) -> PinIn {
        PinIn {
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
            _marker: PhantomData,
        }
    }
}

pub struct Pout0 {
    _marker: PhantomData<*const ()>,
}
pub struct Pout1 {
    _marker: PhantomData<*const ()>,
}
pub struct Pout2 {
    _marker: PhantomData<*const ()>,
}
pub struct Pout3 {
    _marker: PhantomData<*const ()>,
}
pub struct Pout4 {
    _marker: PhantomData<*const ()>,
}

impl GPO {
    pub fn write(&self, val: usize) {
        unsafe {
            gpo::Bits::write(val);
        }
    }

    pub fn split(self) -> PinOut {
        PinOut {
            pout0: Pout0 {
                _marker: PhantomData,
            },
            pout1: Pout1 {
                _marker: PhantomData,
            },
            pout2: Pout2 {
                _marker: PhantomData,
            },

            pout3: Pout3 {
                _marker: PhantomData,
            },

            pout4: Pout4 {
                _marker: PhantomData,
            },

            _marker: PhantomData,
        }
    }
}
pub trait OutputPin: Sized {
    fn set_high(&self);
    fn set_low(&self);
}

impl OutputPin for Pout0 {
    fn set_high(&self) {
        gpo::Pout0::set()
    }
    fn set_low(&self) {
        gpo::Pout0::clear()
    }
}

impl OutputPin for Pout1 {
    fn set_high(&self) {
        gpo::Pout1::set()
    }
    fn set_low(&self) {
        gpo::Pout1::clear()
    }
}

impl OutputPin for Pout2 {
    fn set_high(&self) {
        gpo::Pout2::set()
    }
    fn set_low(&self) {
        gpo::Pout2::clear()
    }
}

impl OutputPin for Pout3 {
    fn set_high(&self) {
        gpo::Pout3::set()
    }
    fn set_low(&self) {
        gpo::Pout3::clear()
    }
}

impl OutputPin for Pout4 {
    fn set_high(&self) {
        gpo::Pout4::set()
    }
    fn set_low(&self) {
        gpo::Pout4::clear()
    }
}

pub trait InputPin: Sized {
    fn is_low(&self) -> bool;
    fn is_high(&self) -> bool;
}

impl InputPin for Pin0 {
    fn is_low(&self) -> bool {
        gpi::Pin0::read() == 0
    }
    fn is_high(&self) -> bool {
        gpi::Pin0::read() == 1
    }
}

impl InputPin for Pin1 {
    fn is_low(&self) -> bool {
        gpi::Pin1::read() == 0
    }
    fn is_high(&self) -> bool {
        gpi::Pin1::read() == 1
    }
}

impl InputPin for Pin2 {
    fn is_low(&self) -> bool {
        gpi::Pin2::read() == 0
    }
    fn is_high(&self) -> bool {
        gpi::Pin2::read() == 1
    }
}

impl InputPin for Pin3 {
    fn is_low(&self) -> bool {
        gpi::Pin3::read() == 0
    }
    fn is_high(&self) -> bool {
        gpi::Pin3::read() == 1
    }
}

impl UART {
    pub fn write_word(&self, val: usize) {
        unsafe {
            uart::write_word::Bits::write(val);
        }
    }

    pub fn write_byte(&self, val: u8) {
        unsafe {
            uart::write_byte::Bits::write(val as usize);
        }
    }
}
use core::fmt::Write;

impl Write for UART {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write_byte(b)
        }
        Ok(())
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
use core::ops::Deref;


pub use interrupt0::Interrupt0;
pub use interrupt1::Interrupt1;
pub use interrupt2::Interrupt2;
pub use interrupt3::Interrupt3;
pub use interrupt4::Interrupt4;
pub use interrupt5::Interrupt5;
pub use interrupt6::Interrupt6;
pub use interrupt7::Interrupt7;

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
        interrupt0::Pending::clear();
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
        interrupt2::Pending::clear();
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


unsafe impl Interrupt for interrupt4::Interrupt4 {
    #[inline(always)]
    unsafe fn pend_int() {
        interrupt4::Pending::set();
    }
    #[inline(always)]
    unsafe fn enable_int() {
        interrupt4::Enabled::set();
    }
    #[inline(always)]
    unsafe fn disable_int() {
        interrupt4::Enabled::clear();
    }
    #[inline(always)]
    unsafe fn set_priority(prio: u8) {
        interrupt4::Priority::set(prio as usize);
    }
    #[inline(always)]
    unsafe fn clear_int() {
        interrupt4::Pending::clear();
    }
}

unsafe impl Interrupt for interrupt5::Interrupt5 {
    #[inline(always)]
    unsafe fn pend_int() {
        interrupt5::Pending::set();
    }
    #[inline(always)]
    unsafe fn enable_int() {
        interrupt5::Enabled::set();
    }
    #[inline(always)]
    unsafe fn disable_int() {
        interrupt5::Enabled::clear();
    }
    #[inline(always)]
    unsafe fn set_priority(prio: u8) {
        interrupt5::Priority::set(prio as usize);
    }
    #[inline(always)]
    unsafe fn clear_int() {
        interrupt5::Pending::clear();
    }
}

unsafe impl Interrupt for interrupt6::Interrupt6 {
    #[inline(always)]
    unsafe fn pend_int() {
        interrupt6::Pending::set();
    }
    #[inline(always)]
    unsafe fn enable_int() {
        interrupt6::Enabled::set();
    }
    #[inline(always)]
    unsafe fn disable_int() {
        interrupt6::Enabled::clear();
    }
    #[inline(always)]
    unsafe fn set_priority(prio: u8) {
        interrupt6::Priority::set(prio as usize);
    }
    #[inline(always)]
    unsafe fn clear_int() {
        interrupt6::Pending::clear();
    }
}

unsafe impl Interrupt for interrupt7::Interrupt7 {
    #[inline(always)]
    unsafe fn pend_int() {
        interrupt7::Pending::set();
    }
    #[inline(always)]
    unsafe fn enable_int() {
        interrupt7::Enabled::set();
    }
    #[inline(always)]
    unsafe fn disable_int() {
        interrupt7::Enabled::clear();
    }
    #[inline(always)]
    unsafe fn set_priority(prio: u8) {
        interrupt7::Priority::set(prio as usize);
    }
    #[inline(always)]
    unsafe fn clear_int() {
        interrupt7::Pending::clear();
    }
}

