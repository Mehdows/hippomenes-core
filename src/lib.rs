#![no_std]
use core::marker::PhantomData;
use csr_trait::CSR;
pub struct Peripherals {
    pub timer: Timer,
    pub gpi: GPI,
    pub gpo: GPO,
    pub uart: UART,
    pub i0_timestamp: I0Timestamp,
    pub interrupt_controller: InterruptController
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
                        word_write: UARTWriteWordReg { _marker: PhantomData,},
                        byte_write: UARTWriteByteReg { _marker: PhantomData,},
                    },
                    i0_timestamp: I0Timestamp {
                        _marker: PhantomData,
                    },
                    interrupt_controller: InterruptController {
                        mstatus: MStatus {_marker: PhantomData,},
                        mintthresh: MIntThresh {_marker: PhantomData,},
                        interrupt0: Interrupt0 {_marker: PhantomData,},
                        interrupt1: Interrupt1 {_marker: PhantomData,},
                        interrupt2: Interrupt2 {_marker: PhantomData,},
                        interrupt3: Interrupt3 {_marker: PhantomData,},
                    }
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
                word_write: UARTWriteWordReg { _marker: PhantomData,},
                byte_write: UARTWriteByteReg { _marker: PhantomData,},
            },
            i0_timestamp: I0Timestamp {
                _marker: PhantomData,
            },
            interrupt_controller: InterruptController {
                mstatus: MStatus {_marker: PhantomData,},
                mintthresh: MIntThresh {_marker: PhantomData,},
                interrupt0: Interrupt0 {_marker: PhantomData,},
                interrupt1: Interrupt1 {_marker: PhantomData,},
                interrupt2: Interrupt2 {_marker: PhantomData,},
                interrupt3: Interrupt3 {_marker: PhantomData,},

            }
        }
    }
}
pub struct I0Timestamp {
    _marker:PhantomData<*const ()>,
}
pub struct GPI {
    _marker:PhantomData<*const ()>,
}
pub struct GPO {
    _marker:PhantomData<*const ()>,
}
pub struct UART {
    pub word_write: UARTWriteWordReg,
    pub byte_write: UARTWriteByteReg,
}
pub struct UARTWriteWordReg {
    _marker: PhantomData<*const ()>,
}
pub struct UARTWriteByteReg {
    _marker: PhantomData<*const ()>,
}
pub struct Timer {
    _marker:PhantomData<*const ()>,
}
pub struct MStatus {
    _marker:PhantomData<*const ()>,
}
pub struct MIntThresh {
    _marker:PhantomData<*const ()>,
}
pub struct Interrupt0 {
    _marker: PhantomData<*const ()>,
}
pub struct Interrupt1 {
    _marker: PhantomData<*const ()>,
}
pub struct Interrupt2 {
    _marker: PhantomData<*const ()>,
}
pub struct Interrupt3 {
    _marker: PhantomData<*const ()>,
}

pub struct InterruptController {
    pub mstatus: MStatus,
    pub mintthresh: MIntThresh,
    pub interrupt0: Interrupt0,
    pub interrupt1: Interrupt1,
    pub interrupt2: Interrupt2,
    pub interrupt3: Interrupt3,
}

impl CSR for I0Timestamp {
    const ADDR: u16 = 0xB40;
}

impl CSR for GPI {
    const ADDR: u16 = 0x001;
}

impl CSR for GPO {
    const ADDR: u16 = 0x000;
}
impl CSR for Timer {
    const ADDR: u16 = 0x400;
}
impl CSR for UARTWriteWordReg {
    const ADDR: u16 = 0x050;
}
impl CSR for UARTWriteByteReg {
    const ADDR: u16 = 0x051;
}
impl CSR for MStatus {
    const ADDR: u16 = 0x300;
}
impl CSR for MIntThresh {
    const ADDR: u16 = 0x347;
}
impl CSR for Interrupt0 {
    const ADDR: u16 = 0xB20;
}
impl CSR for Interrupt1 {
    const ADDR: u16 = 0xB21;
}
impl CSR for Interrupt2 {
    const ADDR: u16 = 0xB22;
}
impl CSR for Interrupt3 {
    const ADDR: u16 = 0xB23;
}

pub unsafe trait Interrupt {
    unsafe fn pend_int(&mut self);
    unsafe fn clear_int(&mut self);
    unsafe fn enable_int(&mut self);
    unsafe fn disable_int(&mut self);
    unsafe fn write_priority(&mut self, prio: u8);
}

unsafe impl Interrupt for Interrupt0 {
    #[inline(always)]
    unsafe fn pend_int(&mut self) {
        self.set(1);
    }
    #[inline(always)]
    unsafe fn enable_int(&mut self) {
        self.set(1<<1);
    }
    #[inline(always)]
    unsafe fn disable_int(&mut self) {
        self.clear(1<<1);
    }
    #[inline(always)]
    unsafe fn write_priority(&mut self, prio: u8) {
        self.write((prio<<2) as u32);
    }
    #[inline(always)]
    unsafe fn clear_int(&mut self) {
        self.clear(1);
    }
}
unsafe impl Interrupt for Interrupt1 {
    #[inline(always)]
    unsafe fn pend_int(&mut self) {
        self.set(1);
    }
    #[inline(always)]
    unsafe fn enable_int(&mut self) {
        self.set(1<<1);
    }
    #[inline(always)]
    unsafe fn disable_int(&mut self) {
        self.clear(1<<1);
    }
    #[inline(always)]
    unsafe fn write_priority(&mut self, prio: u8) {
        self.write((prio<<2) as u32);
    }
    #[inline(always)]
    unsafe fn clear_int(&mut self) {
        self.clear(1);
    }
}
unsafe impl Interrupt for Interrupt2 {
    #[inline(always)]
    unsafe fn pend_int(&mut self) {
        self.set(1);
    }
    #[inline(always)]
    unsafe fn enable_int(&mut self) {
        self.set(1<<1);
    }
    #[inline(always)]
    unsafe fn disable_int(&mut self) {
        self.clear(1<<1);
    }
    #[inline(always)]
    unsafe fn write_priority(&mut self, prio: u8) {
        self.write((prio<<2) as u32);
    }
    #[inline(always)]
    unsafe fn clear_int(&mut self) {
        self.clear(1);
    }
}
unsafe impl Interrupt for Interrupt3 {
    #[inline(always)]
    unsafe fn pend_int(&mut self) {
        self.set(1);
    }
    #[inline(always)]
    unsafe fn enable_int(&mut self) {
        self.set(1<<1);
    }
    #[inline(always)]
    unsafe fn disable_int(&mut self) {
        self.clear(1<<1);
    }
    #[inline(always)]
    unsafe fn write_priority(&mut self, prio: u8) {
        self.write((prio<<2) as u32);
    }
    #[inline(always)]
    unsafe fn clear_int(&mut self) {
        self.clear(1);
    }
}
impl MStatus {
    #[inline(always)]
    pub unsafe fn global_interrupt_enable(&mut self) {
        self.set(8);
    }

    #[inline(always)]
    pub unsafe fn global_interrupt_disable(&mut self) {
        self.clear(8);
    }
}

pub enum Interrupts {
    Interrupt0,
    Interrupt1,
    Interrupt2,
    Interrupt3
}

impl Interrupts {
    pub unsafe fn pend(self, ictl:&mut InterruptController){
        match self {
            Interrupts::Interrupt0 => {ictl.interrupt0.pend_int()},
            Interrupts::Interrupt1 => {ictl.interrupt1.pend_int()},
            Interrupts::Interrupt2 => {ictl.interrupt2.pend_int()},
            Interrupts::Interrupt3 => {ictl.interrupt3.pend_int()},
        }
    }
    pub unsafe fn unpend(self, ictl: &mut InterruptController){
        match self {
            Interrupts::Interrupt0 => {ictl.interrupt0.clear_int()},
            Interrupts::Interrupt1 => {ictl.interrupt1.clear_int()},
            Interrupts::Interrupt2 => {ictl.interrupt2.clear_int()},
            Interrupts::Interrupt3 => {ictl.interrupt3.clear_int()},
        }

    }
}
