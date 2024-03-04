<<<<<<< HEAD
use alloc::{collections::BTreeMap, string::String, sync::Arc};
use spin::RwLock;
use x86_64::{
    structures::paging::{page::PageRange, Page},
    VirtAddr,
=======
use alloc::{collections::BTreeMap, sync::Arc};
use spin::RwLock;
use x86_64::structures::paging::{
    page::{PageRange, PageRangeInclusive},
    Page,
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
};

use super::*;

#[derive(Debug, Clone)]
pub struct ProcessData {
    // shared data
    pub(super) env: Arc<RwLock<BTreeMap<String, String>>>,

    // process specific data
<<<<<<< HEAD
    pub(super) stack_segment: Option<PageRange>,
    pub(super) stack_memory_usage: usize,
=======
    pub(super) stack_segment: Option<PageRange>
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
}

impl Default for ProcessData {
    fn default() -> Self {
        Self {
            env: Arc::new(RwLock::new(BTreeMap::new())),
<<<<<<< HEAD
            stack_segment: None,
            stack_memory_usage: 0,
=======
            stack_segment: None
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
        }
    }
}

impl ProcessData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn env(&self, key: &str) -> Option<String> {
        self.env.read().get(key).cloned()
    }

<<<<<<< HEAD
    pub fn set_env(self, key: &str, val: &str) -> Self {
        self.env.write().insert(key.into(), val.into());
        self
=======
    pub fn set_env(&mut self, key: &str, val: &str) {
        self.env.write().insert(key.into(), val.into());
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }

    pub fn set_stack(&mut self, start: VirtAddr, size: u64) {
        let start = Page::containing_address(start);
        self.stack_segment = Some(Page::range(start, start + size));
<<<<<<< HEAD
        self.stack_memory_usage = size as usize;
    }

    pub fn is_on_stack(&self, addr: VirtAddr) -> bool {
        if let Some(stack_range) = self.stack_segment.as_ref() {
            let addr = addr.as_u64();
            let cur_stack_bot = stack_range.start.start_address().as_u64();
            trace!("Current stack bot: {:#x}", cur_stack_bot);
            trace!("Address to access: {:#x}", addr);
            addr & STACK_START_MASK == cur_stack_bot & STACK_START_MASK
        } else {
            false
        }
=======
    }

    pub fn is_on_stack(&self, addr: VirtAddr) -> bool {
        // FIXME: check if the address is on the stack
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }
}
