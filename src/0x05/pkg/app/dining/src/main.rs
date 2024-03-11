#![no_std]
#![no_main]

use lib::*;

extern crate lib;

static CHOPSTICK: [Semaphore; 5] = semaphore_array![0, 1, 2, 3, 4];
static WAITER: Semaphore = Semaphore::new(5);

fn main() -> isize {
    let mut pids = [0u16; 5];

    for idx in 0..5 {
        CHOPSTICK[idx].init(1);
    }
    WAITER.init(1);

    for idx in 0..5 {
        let pid = sys_fork();
        if pid == 0 {
            philosopher(idx);
        } else {
            pids[idx] = pid;
        }
    }

    for i in 0..5 {
        let id = sys_wait_pid(pids[i]);
        println!("#{} exit.", id);
    }

    0
}

fn philosopher(id: usize) -> ! {
    for _ in 0..20 {
        println!("#{} is sleeping.", id);
        core::hint::spin_loop();

        println!("#{} is thinking.", id);

        WAITER.wait();

        CHOPSTICK[id].wait();
        CHOPSTICK[(id + 1) % 5].wait();

        println!("#{} is eating.", id);

        CHOPSTICK[(id + 1) % 5].signal();
        CHOPSTICK[id].signal();

        WAITER.signal();
    }
    sys_exit(id as isize);
}

entry!(main);
