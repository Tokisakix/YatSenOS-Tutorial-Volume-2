<<<<<<< HEAD
use super::ProcessId;
use super::*;
use crate::memory::*;
use alloc::string::String;
use alloc::sync::Arc;
=======
use super::*;
use crate::memory::*;
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
use alloc::sync::Weak;
use alloc::vec::Vec;
use spin::*;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::page::PageRange;
use x86_64::structures::paging::*;
<<<<<<< HEAD
use x86_64::VirtAddr;
=======
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

#[derive(Clone)]
pub struct Process {
    pid: ProcessId,
    inner: Arc<RwLock<ProcessInner>>,
}

pub struct ProcessInner {
    name: String,
    parent: Option<Weak<Process>>,
    children: Vec<Arc<Process>>,
    ticks_passed: usize,
    status: ProgramStatus,
    exit_code: Option<isize>,
    context: ProcessContext,
    page_table: Option<PageTableContext>,
    proc_data: Option<ProcessData>,
}

impl Process {
    #[inline]
    pub fn pid(&self) -> ProcessId {
        self.pid
    }

    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<ProcessInner> {
        self.inner.write()
    }

    #[inline]
    pub fn read(&self) -> RwLockReadGuard<ProcessInner> {
        self.inner.read()
    }

    pub fn new(
        name: String,
        parent: Option<Weak<Process>>,
        page_table: PageTableContext,
        proc_data: Option<ProcessData>,
    ) -> Arc<Self> {
        let name = name.to_ascii_lowercase();

        // create context
        let pid = ProcessId::new();

        let inner = ProcessInner {
            name,
            parent,
            status: ProgramStatus::Ready,
            context: ProcessContext::default(),
            ticks_passed: 0,
            exit_code: None,
            children: Vec::new(),
            page_table: Some(page_table),
            proc_data: Some(proc_data.unwrap_or_default()),
        };

        trace!("New process {}#{} created.", &inner.name, pid);

        // create process struct
        Arc::new(Self {
            pid,
            inner: Arc::new(RwLock::new(inner)),
        })
    }

    pub fn kill(&self, ret: isize) {
        let mut inner = self.inner.write();

        debug!(
            "Killing process {}#{} with ret code: {}",
            inner.name(),
            self.pid,
            ret
        );

        inner.kill(ret);
    }

    pub fn alloc_init_stack(&self) -> VirtAddr {
<<<<<<< HEAD
        // stack top set by pid
        let offset = (self.pid.0 - 1) as u64 * STACK_MAX_SIZE;
        let stack_top = STACK_INIT_TOP - offset;
        let stack_bottom = STACT_INIT_BOT - offset;

        let stack_top_addr = VirtAddr::new(stack_top);
        let stack_bottom_addr = VirtAddr::new(stack_bottom);
        let alloc = &mut *get_frame_alloc_for_sure();
        let mut mapper = self.read().page_table.as_ref().unwrap().mapper();

        elf::map_range(stack_bottom, STACK_DEF_PAGE, &mut mapper, alloc).unwrap();

        self.write().set_stack(stack_bottom_addr, STACK_DEF_PAGE);

        stack_top_addr
=======
        // FIXME: alloc init stack base on self pid

        VirtAddr::new(0)
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }
}

impl ProcessInner {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tick(&mut self) {
        self.ticks_passed += 1;
    }

    pub fn status(&self) -> ProgramStatus {
        self.status
    }

    pub fn pause(&mut self) {
        self.status = ProgramStatus::Ready;
    }

    pub fn resume(&mut self) {
        self.status = ProgramStatus::Running;
    }

    pub fn exit_code(&self) -> Option<isize> {
        self.exit_code
    }

    pub fn clone_page_table(&self) -> PageTableContext {
<<<<<<< HEAD
        self.page_table.as_ref().unwrap().clone()
=======
        self.page_table.as_ref().unwrap().clone_l4()
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }

    pub fn is_ready(&self) -> bool {
        self.status == ProgramStatus::Ready
    }

    /// Save the process's context
    /// mark the process as ready
    pub(super) fn save(&mut self, context: &ProcessContext) {
<<<<<<< HEAD
        self.context.save(context);

        // dead process should not be ready
        // (kernel thread exit without syscall)
        if self.status == ProgramStatus::Running {
            self.status = ProgramStatus::Ready;
        }
=======
        // FIXME: save the process's context
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }

    /// Restore the process's context
    /// mark the process as running
    pub(super) fn restore(&mut self, context: &mut ProcessContext) {
<<<<<<< HEAD
        self.context.restore(context);
        self.page_table.as_ref().unwrap().load();
        self.status = ProgramStatus::Running;
    }

    pub fn init_stack_frame(&mut self, entry: VirtAddr, stack_top: VirtAddr) {
        self.context.init_stack_frame(entry, stack_top);
=======
        // FIXME: restore the process's context

        // FIXME: restore the process's page table
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }

    pub fn parent(&self) -> Option<Arc<Process>> {
        self.parent.as_ref().and_then(|p| p.upgrade())
    }

<<<<<<< HEAD
    pub fn try_alloc_new_stack_page(&mut self, addr: VirtAddr) -> Result<(), MapToError<Size4KiB>> {
        let alloc = &mut *get_frame_alloc_for_sure();
        let new_start_page = Page::<Size4KiB>::containing_address(addr);
        let old_stack = self.proc_data.as_ref().unwrap().stack_segment.unwrap();

        let pages = old_stack.start - new_start_page;
        let page_table = &mut self.page_table.as_mut().unwrap().mapper();

        trace!(
            "Fill missing pages...[{:#x} -> {:#x}) ({} pages)",
            new_start_page.start_address().as_u64(),
            old_stack.start.start_address().as_u64(),
            pages
        );

        elf::map_range(addr.as_u64(), pages, page_table, alloc)?;

        let new_stack = PageRange {
            start: new_start_page,
            end: old_stack.end,
        };

        let proc_data = self.proc_data.as_mut().unwrap();
        proc_data.stack_memory_usage = new_stack.count();
        proc_data.stack_segment = Some(new_stack);

        Ok(())
    }

    pub fn kill(&mut self, ret: isize) {
        self.status = ProgramStatus::Dead;
        self.exit_code = Some(ret);
        self.proc_data.take();
        self.page_table.take();
=======
    pub fn kill(&mut self, ret: isize) {
        // FIXME: set exit code

        // FIXME: set status to dead

        // FIXME: take and drop unused resources
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }
}

impl core::ops::Deref for Process {
    type Target = Arc<RwLock<ProcessInner>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::Deref for ProcessInner {
    type Target = ProcessData;

    fn deref(&self) -> &Self::Target {
        self.proc_data
            .as_ref()
            .expect("Process data empty. The process may be killed.")
    }
}

impl core::ops::DerefMut for ProcessInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.proc_data
            .as_mut()
            .expect("Process data empty. The process may be killed.")
    }
}

impl core::fmt::Debug for Process {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut f = f.debug_struct("Process");
        f.field("pid", &self.pid);

        let inner = self.inner.read();
        f.field("name", &inner.name);
        f.field("parent", &inner.parent().map(|p| p.pid));
        f.field("status", &inner.status);
        f.field("ticks_passed", &inner.ticks_passed);
        f.field(
            "children",
            &inner.children.iter().map(|c| c.pid.0).collect::<Vec<u16>>(),
        );
        f.field("page_table", &inner.page_table);
        f.field("status", &inner.status);
        f.field("context", &inner.context);
        f.field("stack", &inner.proc_data.as_ref().map(|d| d.stack_segment));
        f.finish()
    }
}

impl core::fmt::Display for Process {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let inner = self.inner.read();
        write!(
            f,
            " #{:-3} | #{:-3} | {:12} | {:7} | {:?}",
<<<<<<< HEAD
            u16::from(self.pid),
            inner.parent().map(|p| u16::from(p.pid)).unwrap_or(0),
=======
            self.pid.0,
            inner.parent().map(|p| p.pid.0).unwrap_or(0),
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
            inner.name,
            inner.ticks_passed,
            inner.status
        )?;
        Ok(())
    }
}
