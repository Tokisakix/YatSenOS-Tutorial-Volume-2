use core::sync::atomic::{AtomicU16, Ordering};

use crate::proc::ProcessId;
use alloc::{string::String, vec::Vec};
use x86::cpuid::CpuId;

<<<<<<< HEAD
const MAX_CPU_COUNT: usize = 8;
=======
const MAX_CPU_COUNT: usize = 4;
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

#[allow(clippy::declare_interior_mutable_const)]
const EMPTY: Processor = Processor::new(); // means no process

static PROCESSORS: [Processor; MAX_CPU_COUNT] = [EMPTY; MAX_CPU_COUNT];

<<<<<<< HEAD
=======
/// Returns the current processor based on the current APIC ID
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
fn current() -> &'static Processor {
    let cpuid = CpuId::new()
        .get_feature_info()
        .unwrap()
        .initial_local_apic_id() as usize;

    &PROCESSORS[cpuid]
}

pub fn print_processors() -> String {
    alloc::format!(
        "CPUs   : {}\n",
        PROCESSORS
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.is_free())
            .map(|(i, p)| alloc::format!("[{}: {}]", i, p.get_pid().unwrap()))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

<<<<<<< HEAD
=======
/// Processor holds the current process id
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
pub struct Processor(AtomicU16);

impl Processor {
    pub const fn new() -> Self {
        Self(AtomicU16::new(0))
    }
}

#[inline]
pub fn set_pid(pid: ProcessId) {
    current().set_pid(pid)
}

#[inline]
<<<<<<< HEAD
pub fn current_pid() -> ProcessId {
=======
pub fn get_pid() -> ProcessId {
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    current().get_pid().expect("No current process")
}

impl Processor {
    #[inline]
    pub fn is_free(&self) -> bool {
        self.0.load(Ordering::Relaxed) == 0
    }

    #[inline]
    pub fn set_pid(&self, pid: ProcessId) {
<<<<<<< HEAD
        self.0.store(u16::from(pid), Ordering::Relaxed);
=======
        self.0.store(pid.0, Ordering::Relaxed);
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }

    #[inline]
    pub fn get_pid(&self) -> Option<ProcessId> {
        let pid = self.0.load(Ordering::Relaxed);
        if pid == 0 {
            None
        } else {
            Some(ProcessId(pid))
        }
    }
}
