use core::sync::atomic::{AtomicU16, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(pub u16);

impl ProcessId {
    pub fn new() -> Self {
<<<<<<< HEAD
        static NEXT_PID: AtomicU16 = AtomicU16::new(1); // pid 0 is reserved for no process
        ProcessId(NEXT_PID.fetch_add(1, Ordering::Relaxed))
=======
        // FIXME: Get a unique PID
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }
}

impl Default for ProcessId {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for ProcessId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Debug for ProcessId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ProcessId> for u16 {
    fn from(pid: ProcessId) -> Self {
        pid.0
    }
}
