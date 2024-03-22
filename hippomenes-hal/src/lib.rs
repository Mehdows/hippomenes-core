#![no_std]
use core::marker::PhantomData;
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
pub struct UART<T> {
    pin: T,
    timer: Timer,
    rate: u32,
    _marker: PhantomData<*const ()>,
}

impl<T: Pin> UART<T> {
    pub fn new(
        pin: T,
        timer: Timer,
        rate: u32,
        //_handler_token: impl InterruptToken,
    ) -> UART<T> {
        unsafe { BAUD = (CLOCK_FREQ / rate) as usize };
        create_uart_token!(pin0);
        unsafe {
            Interrupt0::disable_int();
            Interrupt0::set_priority(7);
        };
        timer.counter_top().write(unsafe { BAUD });
        let u = UART {
            pin,
            timer,
            rate,
            _marker: PhantomData,
        };
        u
    }

    pub fn send(&mut self, buf: [u8; BUF_WIDTH]) {
        unsafe {
            START = true;
            BUFFER = buf;
            BUFFER_PTR = 0;
            BYTE_PTR = 0;
            Interrupt0::enable_int()
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
            let pin = unsafe { Peripherals::steal().gpio.pins().$x };
            let timer = unsafe { Peripherals::steal().timer };
            if START {
                pin.set_low(); //start bit
                START = false;
                timer.counter_top().write(BAUD);
                return;
            }
            if BYTE_PTR == 8 {
                pin.set_high(); //stop bit
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
                    return;
                }
            } else {
                if ((BUFFER[BUFFER_PTR] >> BYTE_PTR) & 0b1) == 1 {
                    BYTE_PTR += 1;
                    pin.set_high()
                } else {
                    BYTE_PTR += 1;
                    pin.set_low()
                }
            }
        }

        pub struct UARTHandlerToken;

        unsafe impl InterruptToken<UART<Pin0>> for UARTHandlerToken {}

        UARTHandlerToken
    }};
}

pub unsafe trait InterruptToken<T> {}
