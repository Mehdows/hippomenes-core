#![no_std]
use hippomenes_core::*;

const CLOCK_FREQ: u32 = 20_000_000;
const INTERVAL: u32 = 10000; //1000 cycles break between chars
                             //static mut UART: Option<UART<Pin0>> = None;
const BUF_WIDTH: usize = 8;
static mut BUFFER: [u8; BUF_WIDTH] = [0; BUF_WIDTH];
static mut BUFFER_PTR: usize = 0;
static mut BYTE_PTR: usize = 0;
static mut START: bool = true;
static mut BAUD: usize = 0;

pub struct UART;

impl UART {
    pub fn new(
        _pout: Pout4, // TX (target side) RX (host side)
        timer: Timer,
        rate: u32,
        //_handler_token: impl InterruptToken,
    ) -> UART {
        unsafe { BAUD = (CLOCK_FREQ / rate) as usize };
        create_uart_token!(pout);
        unsafe {
            Interrupt0::disable_int();
            Interrupt0::set_priority(7);
        };
        timer.counter_top().write(unsafe { BAUD });
        UART {}
    }

    pub fn send(&mut self, buf: [u8; BUF_WIDTH]) {
        unsafe {
            START = true;
            BUFFER = buf;
            BUFFER_PTR = 0;
            BYTE_PTR = 0;
            Interrupt0::enable_int() // Timer interrupt
        };
    }
}

#[macro_export]
macro_rules! create_uart_token {
    ($x: ident) => {{
        #[no_mangle]
        #[allow(non_snake_case)]
        fn Interrupt0() {
            unsafe { on_timer_interrupt() };
        }
        #[inline(always)]
        unsafe fn on_timer_interrupt() {
            let p = unsafe { Peripherals::steal() };
            let timer = p.timer;
            let tx = p.gpo.split().pout4;

            if START {
                timer.counter_top().write(BAUD);
                tx.set_low(); //start bit
                START = false;
                return;
            }
            if BYTE_PTR == 8 {
                tx.set_high(); //stop bit
                START = true;
                if BUFFER_PTR < BUF_WIDTH - 1 {
                    BUFFER_PTR += 1;
                    BYTE_PTR = 0;
                    timer.counter_top().write(INTERVAL as usize);
                    return;
                } else {
                    BUFFER_PTR = 0;
                    BYTE_PTR = 0;
                    Interrupt0::disable_int();
                    Interrupt2::pend_int();
                    return;
                }
            } else {
                if ((BUFFER[BUFFER_PTR] >> BYTE_PTR) & 0b1) == 1 {
                    BYTE_PTR += 1;
                    tx.set_high()
                } else {
                    BYTE_PTR += 1;
                    tx.set_low()
                }
            }
        }
    }};
}

pub unsafe trait InterruptToken<T> {}
