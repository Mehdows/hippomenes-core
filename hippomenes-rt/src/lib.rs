#![no_std]
use core::arch::global_asm;
use hippomenes_core::{i0_vec, i1_vec, i2_vec, i8_vec};
pub use hippomenes_rt_macros::entry;
#[no_mangle]
pub unsafe fn _setup_interrupts() {
    i0_vec::Bits::write((Interrupt0 as *const fn() as usize) >> 2);
    i1_vec::Bits::write((Interrupt1 as *const fn() as usize) >> 2);
    i2_vec::Bits::write((Interrupt2 as *const fn() as usize) >> 2);
    i8_vec::Bits::write((_memex as *const fn() as usize) >> 2);
}

extern "C" {
    fn Interrupt0();
    fn Interrupt1();
    fn Interrupt2();
    fn _memex();
}

#[allow(non_snake_case)]
#[no_mangle]
fn DefaultInterruptHandler() {
    loop {}
}

global_asm!(
    "

    .section .init, \"ax\"
    .global _start

_start:
_abs_start:
    .option norelax
    .cfi_startproc
    .cfi_undefined ra
    .option push
    // init global pointer
    la gp, __global_pointer$
    .option pop
    // init stack pointer
    la t1, _stack_start
    andi sp, t1, -16 // align stack to 16-bytes
    // Copy .data from flash to RAM
    la t0, _sdata       //start data
    la t2, _edata       //end data
    la t1, _sidata      //source data
    bgeu t0, t2, 2f
1:  lw t3, 0(t1)
    addi t1, t1, 4
    sw t3, 0(t0)
    addi t0, t0, 4
    bltu t0, t2, 1b
2:  // Zero out .bss
    la t0, _sbss
    la t2, _ebss
    bgeu  t0, t2, 4f
3:  sw  zero, 0(t0)
    addi t0, t0, 4
    bltu t0, t2, 3b
4: // RAM initilized
    call _setup_interrupts
    jal zero, main
    .cfi_endproc
"
);
global_asm!(
    "
.weak DefaultHandler
DefaultHandler:
    j DefaultHandler
    "
);
#[rustfmt::skip]
global_asm!(
    ".section .text.abort
    .global abort
abort:  // make sure there is an abort symbol when linking
    j abort"
);
