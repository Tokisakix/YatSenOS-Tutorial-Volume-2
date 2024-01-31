use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::proc::ProcessContext;
use super::consts::*;

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt[Interrupts::IrqBase as usize + Irq::Timer as usize]
        .set_handler_fn(clock_handler);
}

pub extern "C" fn clock(mut context: ProcessContext) {
    crate::proc::switch(&mut context);
    super::ack();
    // crate::proc::print_process_list();
}

as_handler!(clock);