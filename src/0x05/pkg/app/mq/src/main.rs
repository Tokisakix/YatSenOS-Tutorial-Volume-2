#![no_std]
#![no_main]

use lib::*;

extern crate lib;

const THREAD_NUM: usize = 16;
static mut COUNT: usize = 0;
static NOT_FULL: Semaphore = Semaphore::new(0);
static NOT_EMPTY: Semaphore = Semaphore::new(1);
static MUTEX: Semaphore = Semaphore::new(2);

fn main() -> isize {
    NOT_EMPTY.init(0);
    NOT_FULL.init(THREAD_NUM * 2);
    MUTEX.init(1);

    let mut pids = [0u16; THREAD_NUM];

    for idx in 0..THREAD_NUM {
        let pid = sys_fork();
        if pid == 0 {
            if idx % 2 == 0 {
                producer(idx);
            } else {
                consumer(idx);
            }
        } else {
            pids[idx] = pid;
        }
    }

    for idx in 0..THREAD_NUM {
        let id = sys_wait_pid(pids[idx]);
        println!("#{} exit.", id);
    }

    MUTEX.free();
    NOT_EMPTY.free();
    NOT_FULL.free();

    0
}

fn producer(id: usize) -> ! {
    for _ in 0..10 {
        NOT_FULL.wait();
        MUTEX.wait();
        unsafe { COUNT += 1; }
        println!("#{} produce, Num = {}", id, unsafe { COUNT });
        MUTEX.signal();
        NOT_EMPTY.signal();
    }
    sys_exit(id as isize);
}

fn consumer(id: usize) -> ! {
    for _ in 0..10 {
        NOT_EMPTY.wait();
        MUTEX.wait();
        unsafe { COUNT -= 1; }
        println!("#{} consume, Num = {}", id, unsafe { COUNT });
        MUTEX.signal();
        NOT_FULL.signal();
    }
    sys_exit(id as isize);
}

entry!(main);
