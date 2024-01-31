use crate::memory::*;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt.divide_error.set_handler_fn(divide_error_handler);
    idt.double_fault
        .set_handler_fn(double_fault_handler)
        .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    idt.page_fault
        .set_handler_fn(page_fault_handler)
        .set_stack_index(gdt::PAGE_FAULT_IST_INDEX);

    // TODO: you should handle more exceptions here
    // especially gerneral protection fault (GPF)
    // see: https://wiki.osdev.org/Exceptions
    // idt.debug.set_handler_fn(generalf_error_handler);
    // idt.non_maskable_interrupt.set_handler_fn(generalf_error_handler);
    // idt.breakpoint.set_handler_fn(generalf_error_handler);
    // idt.overflow.set_handler_fn(generalf_error_handler);
    // idt.bound_range_exceeded.set_handler_fn(generalf_error_handler);
    // idt.invalid_opcode.set_handler_fn(generalf_error_handler);
    // idt.device_not_available.set_handler_fn(generalf_error_handler);
    // idt.invalid_tss.set_handler_fn(generalfu_error_handler);
    // idt.segment_not_present.set_handler_fn(generalfu_error_handler);
    // idt.stack_segment_fault.set_handler_fn(generalfu_error_handler);
    // idt.general_protection_fault.set_handler_fn(generalfu_error_handler);
    // idt.x87_floating_point.set_handler_fn(generalf_error_handler);
    // idt.alignment_check.set_handler_fn(generalfu_error_handler);
    // idt.machine_check.set_handler_fn(generalf1_error_handler);
    // idt.simd_floating_point.set_handler_fn(generalf_error_handler);
    // idt.virtualization.set_handler_fn(generalf_error_handler);
    // idt.vmm_communication_exception.set_handler_fn(generalfu_error_handler);
    // idt.security_exception.set_handler_fn(generalfu_error_handler);
}

pub extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: DIVIDE ERROR\n\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT, ERROR_CODE: 0x{:016x}\n\n{:#?}",
        error_code, stack_frame
    );
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    err_code: PageFaultErrorCode,
) {
    panic!(
        "EXCEPTION: PAGE FAULT, ERROR_CODE: {:?}\n\nTrying to access: {:#x}\n{:#?}",
        err_code,
        Cr2::read(),
        stack_frame
    );
}

pub extern "x86-interrupt" fn generalf_error_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: GENERAL ERROR\n\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn generalf1_error_handler(stack_frame: InterruptStackFrame) -> !{
    panic!(
        "EXCEPTION: GENERAL FAULT\n\n{:#?}",
        stack_frame
    );
}

pub extern "x86-interrupt" fn generalfu_error_handler(
    stack_frame: InterruptStackFrame,
    err_code: u64,
) {
    panic!("EXCEPTION: GENERAL ERROR, ERROR_CODE: {:?}\n\n{:#?}", err_code, stack_frame);
}