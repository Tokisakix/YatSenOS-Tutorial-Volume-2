use crate::{memory::gdt, proc::*};
use alloc::format;
<<<<<<< HEAD
use core::convert::TryFrom;
use syscall_def::Syscall;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

mod service;
use super::consts;
use service::*;

pub unsafe fn reg_idt(idt: &mut InterruptDescriptorTable) {
    idt[consts::Interrupts::Syscall as usize]
        .set_handler_fn(syscall_handler)
        .set_stack_index(gdt::SYSCALL_IST_INDEX)
        .set_privilege_level(x86_64::PrivilegeLevel::Ring3);
=======
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// NOTE: import `ysos_syscall` package as `syscall_def` in Cargo.toml
use syscall_def::Syscall;

mod service;
use super::consts;

// FIXME: write syscall service handler in `service.rs`
use service::*;

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    // FIXME: register syscall handler to IDT
    //        - standalone syscall stack
    //        - ring 3
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
}

pub extern "C" fn syscall(mut context: ProcessContext) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        super::syscall::dispatcher(&mut context);
    });
}

as_handler!(syscall);

#[derive(Clone, Debug)]
pub struct SyscallArgs {
    pub syscall: Syscall,
    pub arg0: usize,
    pub arg1: usize,
    pub arg2: usize,
}

pub fn dispatcher(context: &mut ProcessContext) {
    let args = super::syscall::SyscallArgs::new(
<<<<<<< HEAD
        Syscall::try_from(context.regs.rax).unwrap(),
=======
        Syscall::from(context.regs.rax),
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
        context.regs.rdi,
        context.regs.rsi,
        context.regs.rdx,
    );

    match args.syscall {
<<<<<<< HEAD
        // fd: arg0 as u8, buf: &[u8] (arg1 as *const u8, arg2 as len)
        Syscall::Read => context.set_rax(sys_read(&args)),
        // fd: arg0 as u8, buf: &[u8] (arg1 as *const u8, arg2 as len)
        Syscall::Write => context.set_rax(sys_write(&args)),

        // None -> pid: u16
        Syscall::GetPid => context.set_rax(sys_get_pid() as usize),

        // path: &str (arg0 as *const u8, arg1 as len) -> pid: u16
        Syscall::Spawn => context.set_rax(spawn_process(&args)),
        // pid: arg0 as u16
        Syscall::Exit => exit_process(&args, context),
        // pid: arg0 as u16 -> status: isize
        Syscall::WaitPid => context.set_rax(sys_wait_pid(&args)),
        // pid: arg0 as u16
        Syscall::Kill => sys_kill(&args, context),

        // None -> time: usize
        Syscall::Time => context.set_rax(sys_clock() as usize),
        // None
        Syscall::Stat => list_process(),
        // None
        Syscall::ListApp => list_app(),
=======
        // fd: arg0 as u8, buf: &[u8] (ptr: arg1 as *const u8, len: arg2)
        Syscall::Read => { /* FIXME: read from fd & return length */},
        // fd: arg0 as u8, buf: &[u8] (ptr: arg1 as *const u8, len: arg2)
        Syscall::Write => { /* FIXME: write to fd & return length */},

        // None -> pid: u16
        Syscall::GetPid => { /* FIXME: get current pid */ },

        // path: &str (ptr: arg0 as *const u8, len: arg1) -> pid: u16
        Syscall::Spawn => { /* FIXME: spawn process from name */},
        // ret: arg0 as isize
        Syscall::Exit => { /* FIXME: exit process with retcode */},
        // pid: arg0 as u16 -> status: isize
        Syscall::WaitPid => { /* FIXME: check if the process is running or get retcode */},

        // None
        Syscall::Stat => { /* FIXME: list processes */ },
        // None
        Syscall::ListApp => { /* FIXME: list avaliable apps */},

        // ----------------------------------------------------
        // NOTE: following syscall examples are implemented
        // ----------------------------------------------------
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

        // layout: arg0 as *const Layout -> ptr: *mut u8
        Syscall::Allocate => context.set_rax(sys_allocate(&args)),
        // ptr: arg0 as *mut u8
        Syscall::Deallocate => sys_deallocate(&args),
<<<<<<< HEAD
        // None
        Syscall::None => {}
=======
        // Unknown
        Syscall::Unknown => warn!("Unhandled syscall: {:x?}", context.regs.rax),
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }
}

impl SyscallArgs {
    pub fn new(syscall: Syscall, arg0: usize, arg1: usize, arg2: usize) -> Self {
        Self {
            syscall,
            arg0,
            arg1,
            arg2,
        }
    }
}

impl core::fmt::Display for SyscallArgs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "SYSCALL: {:<10} (0x{:016x}, 0x{:016x}, 0x{:016x})",
            format!("{:?}", self.syscall),
            self.arg0,
            self.arg1,
            self.arg2
        )
    }
}
