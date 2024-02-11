use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{memory::gdt, proc::ProcessContext};
use super::consts::*;

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt[Interrupts::IrqBase as usize + Irq::Timer as usize]
        .set_handler_fn(clock_handler)
        .set_stack_index(gdt::CONTEXT_SWITCH_IST_INDEX);
}

pub extern "C" fn clock(mut context: ProcessContext) {
    crate::proc::switch(&mut context);
    super::ack();
}

as_handler!(clock);