use super::*;
use crate::memory::{
    self,
    allocator::{ALLOCATOR, HEAP_SIZE},
    get_frame_alloc_for_sure, PAGE_SIZE,
};
use alloc::collections::BTreeMap;
use alloc::{collections::VecDeque, format, sync::Arc};
use spin::{Mutex, RwLock};
use x86::current;
use x86_64::{registers::control::Cr3, VirtAddr};

pub static PROCESS_MANAGER: spin::Once<ProcessManager> = spin::Once::new();

pub fn init(init: Arc<Process>) {

    // set init process as Running
    init.write().resume();

    // set processor's current pid to init's pid
    processor::set_pid(init.pid());

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
        self.get_proc(&processor::get_pid())
            .expect("No current process")
    }

    pub fn wait_pid(&self, pid: ProcessId) -> isize {
        self.get_proc(&pid)
            .and_then(|p| p.read().exit_code())
            .unwrap_or(-1)
    }

    pub fn save_current(&self, context: &ProcessContext) {
        // update current process's tick count
        let bing_current = self.current();
        let pid = bing_current.pid();
        let mut current = bing_current.write();
        current.tick();

        // update current process's context
        current.save(context);

        // push current process to ready queue if still alive
        self.push_ready(pid);
    }

    pub fn switch_next(&self, context: &mut ProcessContext) -> ProcessId {
        // fetch the next process from ready queue
        // check if the next process is ready,
        //        continue to fetch if not ready
        // restore next process's context
        // update processor's current pid
        let mut pid = processor::get_pid();

        while let Some(next_pid) = self.ready_queue.lock().pop_front() {
            let bing_processes = self.processes.read();
            let proc = bing_processes.get(&next_pid).unwrap();

            if !proc.read().is_ready() {
                debug!("Process #{} is {:?}", next_pid, proc.read().status());
                continue;
            }

            if pid != next_pid {
                proc.write().restore(context);
                processor::set_pid(next_pid);
                pid = next_pid;
            }

            break;
        }

        pid
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

        // alloc stack for the new process base on pid
        let stack_top = proc.alloc_init_stack();

        // set the stack frame
        let mut inner = proc.write();
        inner.pause();
        inner.init_stack_frame(entry, stack_top);
        let pid = proc.pid();
        info!("Spawn process: {}#{}", inner.name(), pid);
        drop(inner);

        // add to process map
        self.add_proc(pid, proc);

        // push to ready queue
        self.push_ready(pid);

        pid
    }

    pub fn kill_current(&self, ret: isize) {
        self.kill(processor::get_pid(), ret);
    }

    pub fn handle_page_fault(&self, addr: VirtAddr, err_code: PageFaultErrorCode) -> bool {
        // FIXME: handle page fault

        false
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

        for (_, p) in self.processes.read().iter() {
            if p.read().status() != ProgramStatus::Dead {
                output += format!("{}\n", p).as_str();
            }
        }

        // TODO: print memory usage of kernel heap

        output += format!("Queue  : {:?}\n", self.ready_queue.lock()).as_str();

        output += &processor::print_processors();

        print!("{}", output);
    }
}
