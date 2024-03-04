use super::*;
use crate::memory::{
    self,
    allocator::{ALLOCATOR, HEAP_SIZE},
    get_frame_alloc_for_sure, PAGE_SIZE,
};
<<<<<<< HEAD
use alloc::collections::BTreeMap;
use alloc::{collections::VecDeque, format, sync::Arc};
use spin::{Mutex, RwLock};
use x86_64::VirtAddr;
=======
use alloc::{collections::*, format};
use spin::{Mutex, RwLock};
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

pub static PROCESS_MANAGER: spin::Once<ProcessManager> = spin::Once::new();

pub fn init(init: Arc<Process>) {
<<<<<<< HEAD
    init.write().resume();
    processor::set_pid(init.pid());
=======

    // FIXME: set init process as Running

    // FIXME: set processor's current pid to init's pid

>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    PROCESS_MANAGER.call_once(|| ProcessManager::new(init));
}

pub fn get_process_manager() -> &'static ProcessManager {
    PROCESS_MANAGER
        .get()
        .expect("Process Manager has not been initialized")
}

pub struct ProcessManager {
    processes: RwLock<BTreeMap<ProcessId, Arc<Process>>>,
    ready_queue: Mutex<VecDeque<ProcessId>>,
}

impl ProcessManager {
    pub fn new(init: Arc<Process>) -> Self {
        let mut processes = BTreeMap::new();
        let ready_queue = VecDeque::new();
        let pid = init.pid();

        trace!("Init {:#?}", init);

        processes.insert(pid, init);
        Self {
            processes: RwLock::new(processes),
            ready_queue: Mutex::new(ready_queue),
        }
    }

    #[inline]
    pub fn push_ready(&self, pid: ProcessId) {
        self.ready_queue.lock().push_back(pid);
    }

    #[inline]
    fn add_proc(&self, pid: ProcessId, proc: Arc<Process>) {
        self.processes.write().insert(pid, proc);
    }

    #[inline]
    fn get_proc(&self, pid: &ProcessId) -> Option<Arc<Process>> {
        self.processes.read().get(pid).cloned()
    }

    pub fn current(&self) -> Arc<Process> {
<<<<<<< HEAD
        self.get_proc(&processor::current_pid())
            .expect("No current process")
    }

    pub fn wait_pid(&self, pid: ProcessId) -> isize {
        self.get_proc(&pid)
            .and_then(|p| p.read().exit_code())
            .unwrap_or(-1)
    }

    pub fn save_current(&self, context: &ProcessContext) {
        let current = self.current();
        let pid = current.pid();

        let mut inner = current.write();
        inner.tick();
        inner.save(context);
        let status = inner.status();
        drop(inner);

        // debug!("Save process {} #{}", current.name(), pid);

        if status != ProgramStatus::Dead {
            self.push_ready(pid);
        } else {
            debug!("Process {:#?} #{} is dead", current, pid);
        }
    }

    pub fn switch_next(&self, context: &mut ProcessContext) -> ProcessId {
        let mut pid = processor::current_pid();

        while let Some(next) = self.ready_queue.lock().pop_front() {
            let map = self.processes.read();
            let proc = map.get(&next).expect("Process not found");

            if !proc.read().is_ready() {
                debug!("Process #{} is {:?}", next, proc.read().status());
                continue;
            }

            // debug!("Switch process {} #{}", proc.read().name(), next);

            if pid != next {
                proc.write().restore(context);
                processor::set_pid(next);
                pid = next;
            }

            break;
        }

        pid
=======
        self.get_proc(&processor::get_pid())
            .expect("No current process")
    }

    pub fn save_current(&self, context: &ProcessContext) {
        // FIXME: update current process's tick count

        // FIXME: update current process's context

        // FIXME: push current process to ready queue if still alive
    }

    pub fn switch_next(&self, context: &mut ProcessContext) -> ProcessId {

        // FIXME: fetch the next process from ready queue

        // FIXME: check if the next process is ready,
        //        continue to fetch if not ready

        // FIXME: restore next process's context

        // FIXME: update processor's current pid

        KERNEL_PID
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }

    pub fn spawn_kernel_thread(
        &self,
        entry: VirtAddr,
        name: String,
        proc_data: Option<ProcessData>,
    ) -> ProcessId {
        let kproc = self.get_proc(&KERNEL_PID).unwrap();
        let page_table = kproc.read().clone_page_table();
        let proc = Process::new(name, Some(Arc::downgrade(&kproc)), page_table, proc_data);

<<<<<<< HEAD
        let stack_top = proc.alloc_init_stack();
        let mut inner = proc.write();
        inner.pause();
        inner.init_stack_frame(entry, stack_top);

        let pid = proc.pid();
        info!("Spawn process: {}#{}", inner.name(), pid);
        drop(inner);

        self.add_proc(pid, proc);
        self.push_ready(pid);

        pid
    }

    pub fn kill_current(&self, ret: isize) {
        self.kill(processor::current_pid(), ret);
    }

    pub fn handle_page_fault(&self, addr: VirtAddr, err_code: PageFaultErrorCode) -> bool {
        if !err_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION) {
            let cur_proc = self.current();
            trace!(
                "Page Fault! Checking if {:#x} is on current process's stack",
                addr
            );
            if cur_proc.read().is_on_stack(addr) {
                cur_proc.write().try_alloc_new_stack_page(addr).is_ok()
            } else {
                false
            }
        } else {
            false
        }
=======
        // alloc stack for the new process base on pid
        let stack_top = proc.alloc_init_stack();

        // FIXME: set the stack frame

        // FIXME: add to process map

        // FIXME: push to ready queue

        KERNEL_PID
    }

    pub fn kill_current(&self, ret: isize) {
        self.kill(processor::get_pid(), ret);
    }

    pub fn handle_page_fault(&self, addr: VirtAddr, err_code: PageFaultErrorCode) -> bool {
        // FIXME: handle page fault

        false
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    }

    pub fn kill(&self, pid: ProcessId, ret: isize) {
        let proc = self.get_proc(&pid);

        if proc.is_none() {
            warn!("Process #{} not found.", pid);
            return;
        }

        let proc = proc.unwrap();

        if proc.read().status() == ProgramStatus::Dead {
            warn!("Process #{} is already dead.", pid);
            return;
        }

        trace!("Kill {:#?}", &proc);

        proc.kill(ret);
    }

    pub fn print_process_list(&self) {
        let mut output = String::from("  PID | PPID | Process Name |  Ticks  | Status\n");
<<<<<<< HEAD
=======

>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
        for (_, p) in self.processes.read().iter() {
            if p.read().status() != ProgramStatus::Dead {
                output += format!("{}\n", p).as_str();
            }
        }

<<<<<<< HEAD
        let heap_used = ALLOCATOR.lock().used();
        let heap_size = HEAP_SIZE;

        let alloc = get_frame_alloc_for_sure();
        let frames_used = alloc.frames_used();
        let frames_total = alloc.frames_total();

        let (sys_used, sys_used_unit) = memory::humanized_size(heap_used as u64);
        let (sys_size, sys_size_unit) = memory::humanized_size(heap_size as u64);

        output += format!(
            "Kernel : {:>6.*} {} / {:>6.*} {} ({:>5.2}%)\n",
            2,
            sys_used,
            sys_used_unit,
            2,
            sys_size,
            sys_size_unit,
            heap_used as f64 / heap_size as f64 * 100.0
        )
        .as_str();

        // put used/total frames in MiB
        let (used_size, used_unit) = memory::humanized_size(frames_used as u64 * PAGE_SIZE);
        let (tot_size, tot_unit) = memory::humanized_size(frames_total as u64 * PAGE_SIZE);

        output += format!(
            "Memory : {:>6.*} {} / {:>6.*} {} ({:>5.2}%)\n",
            2,
            used_size,
            used_unit,
            2,
            tot_size,
            tot_unit,
            frames_used as f64 / frames_total as f64 * 100.0
        )
        .as_str();
=======
        // TODO: print memory usage of kernel heap
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

        output += format!("Queue  : {:?}\n", self.ready_queue.lock()).as_str();

        output += &processor::print_processors();

        print!("{}", output);
    }
}
