use alloc::vec;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{input::push_key, serial::get_serial_for_sure};
use pc_keyboard::DecodedKey;
use super::consts::*;

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt[Interrupts::IrqBase as usize + Irq::Serial0 as usize]
        .set_handler_fn(serial_handler);
}

pub extern "x86-interrupt" fn serial_handler(_st: InterruptStackFrame) {
    receive();
    super::ack();
}

/// Receive character from uart 16550
/// Should be called on every interrupt
fn receive() {
    // receive character from uart 16550, put it into INPUT_BUFFER
    let mut buf = vec::Vec::with_capacity(4);
    while let Some(scancode) = get_serial_for_sure().receive() {
        match scancode {
            127 => push_key(DecodedKey::Unicode('\x08')),
            13 => push_key(DecodedKey::Unicode('\n')),
            c => {
                buf.push(c);

                if let Ok(s) = core::str::from_utf8(&buf) {
                    let chr = s.chars().next().unwrap();
                    push_key(DecodedKey::Unicode(chr));
                    buf.clear();
                }
            }
        }
    }
}