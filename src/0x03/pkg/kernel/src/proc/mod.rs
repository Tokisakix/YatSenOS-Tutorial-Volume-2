mod context;
mod data;
mod manager;
mod paging;
mod pid;
mod process;
mod processor;

use manager::*;
<<<<<<< HEAD
use paging::*;
use process::*;

use alloc::string::String;
pub use context::ProcessContext;
=======
use process::*;
use crate::memory::PAGE_SIZE;

use alloc::string::String;
pub use context::ProcessContext;
pub use paging::PageTableContext;
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
pub use data::ProcessData;
pub use pid::ProcessId;

use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::VirtAddr;

// 0xffff_ff00_0000_0000 is the kernel's address space
pub const STACK_MAX: u64 = 0x0000_4000_0000_0000;
<<<<<<< HEAD
// stack max addr, every thread has a stack space
// from 0x????_????_0000_0000 to 0x????_????_ffff_ffff
// 0x100000000 bytes -> 4GiB
// allow 0x2000 (4096) threads run as a time
// 0x????_2000_????_???? -> 0x????_3fff_????_????
// init alloc stack has size of 0x2000 (2 frames)
// every time we meet a page fault, we alloc more frames
pub const STACK_MAX_PAGES: u64 = 0x100000;
pub const STACK_MAX_SIZE: u64 = STACK_MAX_PAGES * crate::memory::PAGE_SIZE;
pub const STACK_START_MASK: u64 = !(STACK_MAX_SIZE - 1);
// [bot..0x2000_0000_0000..top..0x3fff_ffff_ffff]
// init stack
pub const STACK_DEF_BOT: u64 = STACK_MAX - STACK_MAX_SIZE;
pub const STACK_DEF_PAGE: u64 = 1;
pub const STACK_DEF_SIZE: u64 = STACK_DEF_PAGE * crate::memory::PAGE_SIZE;
pub const STACT_INIT_BOT: u64 = STACK_MAX - STACK_DEF_SIZE;
=======

pub const STACK_MAX_PAGES: u64 = 0x100000;
pub const STACK_MAX_SIZE: u64 = STACK_MAX_PAGES * PAGE_SIZE;
pub const STACK_START_MASK: u64 = !(STACK_MAX_SIZE - 1);
// [bot..0x2000_0000_0000..top..0x3fff_ffff_ffff]
// init stack
pub const STACK_DEF_PAGE: u64 = 1;
pub const STACK_DEF_SIZE: u64 = STACK_DEF_PAGE * PAGE_SIZE;
pub const STACK_INIT_BOT: u64 = STACK_MAX - STACK_DEF_SIZE;
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
pub const STACK_INIT_TOP: u64 = STACK_MAX - 8;
// [bot..0xffffff0100000000..top..0xffffff01ffffffff]
// kernel stack
pub const KSTACK_MAX: u64 = 0xffff_ff02_0000_0000;
<<<<<<< HEAD
pub const KSTACK_DEF_BOT: u64 = KSTACK_MAX - STACK_MAX_SIZE;
pub const KSTACK_DEF_PAGE: u64 = 8;
pub const KSTACK_DEF_SIZE: u64 = KSTACK_DEF_PAGE * crate::memory::PAGE_SIZE;

=======
pub const KSTACK_DEF_PAGE: u64 = /* FIXME: decide on the boot config */;
pub const KSTACK_DEF_SIZE: u64 = KSTACK_DEF_PAGE * PAGE_SIZE;
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
pub const KSTACK_INIT_BOT: u64 = KSTACK_MAX - KSTACK_DEF_SIZE;
pub const KSTACK_INIT_TOP: u64 = KSTACK_MAX - 8;

pub const KERNEL_PID: ProcessId = ProcessId(1);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProgramStatus {
    Running,
    Ready,
    Blocked,
    Dead,
}

/// init process manager
pub fn init() {
<<<<<<< HEAD
    let stack_bot = VirtAddr::new(KSTACK_INIT_BOT);
    let mut kproc_data = ProcessData::new();
    kproc_data.set_stack(stack_bot, KSTACK_DEF_PAGE);
=======
    let mut kproc_data = ProcessData::new();

    // FIXME: set the kernel stack
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

    trace!("Init process data: {:#?}", kproc_data);

    // kernel process
<<<<<<< HEAD
    let kproc = Process::new(
        String::from("kernel"),
        None,
        PageTableContext::new(),
        Some(kproc_data),
    );

=======
    let kproc = { /* FIXME: create kernel process */ };
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    manager::init(kproc);

    info!("Process Manager Initialized.");
}

pub fn switch(context: &mut ProcessContext) {
    x86_64::instructions::interrupts::without_interrupts(|| {
<<<<<<< HEAD
        let manager = get_process_manager();
        manager.save_current(context);
        manager.switch_next(context);
=======
        // FIXME: switch to the next process
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    });
}

pub fn spawn_kernel_thread(entry: fn() -> !, name: String, data: Option<ProcessData>) -> ProcessId {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let entry = VirtAddr::new(entry as usize as u64);
        get_process_manager().spawn_kernel_thread(entry, name, data)
    })
}

pub fn print_process_list() {
    x86_64::instructions::interrupts::without_interrupts(|| {
        get_process_manager().print_process_list();
    })
}

<<<<<<< HEAD
pub fn process_exit(ret: isize) -> ! {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let manager = get_process_manager();
        manager.kill_current(ret);
    });

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

pub fn env(key: &str) -> Option<String> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        get_process_manager().current().read().env(key)
    })
}

pub fn wait_pid(pid: ProcessId) -> isize {
    x86_64::instructions::interrupts::without_interrupts(|| get_process_manager().wait_pid(pid))
=======
pub fn env(key: &str) -> Option<String> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        // FIXME: get current process's environment variable
    })
}

pub fn process_exit(ret: isize) -> ! {
    x86_64::instructions::interrupts::without_interrupts(|| {
        get_process_manager().kill_current(ret);
    });

    loop {
        x86_64::instructions::hlt();
    }
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
}

pub fn handle_page_fault(addr: VirtAddr, err_code: PageFaultErrorCode) -> bool {
    x86_64::instructions::interrupts::without_interrupts(|| {
        get_process_manager().handle_page_fault(addr, err_code)
    })
}
<<<<<<< HEAD

pub fn current_process_info() {
    debug!("{:#?}", get_process_manager().current())
}
=======
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
