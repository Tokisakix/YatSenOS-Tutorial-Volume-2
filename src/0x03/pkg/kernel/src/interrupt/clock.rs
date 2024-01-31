use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use super::consts::*;

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt[Interrupts::IrqBase as usize + Irq::Timer as usize]
        .set_handler_fn(clock_handler);
}

pub extern "x86-interrupt" fn clock_handler(_sf: InterruptStackFrame) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        if inc_counter() % 0x10000 == 0 {
            // info!("Tick! @{}", read_counter());
        }
        super::ack();
    });
}

static mut COUNTER: u64 = 0;

#[inline]
pub fn read_counter() -> u64 {
    // load counter value
    unsafe {
        COUNTER
    }
}

#[inline]
pub fn inc_counter() -> u64 {
    // read counter value and increase it
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}