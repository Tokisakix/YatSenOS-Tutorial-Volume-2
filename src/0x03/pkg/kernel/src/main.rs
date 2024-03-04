#![no_std]
#![no_main]

use ysos::*;
use ysos_kernel as ysos;

extern crate alloc;

boot::entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static boot::BootInfo) -> ! {
    ysos::init(boot_info);

<<<<<<< HEAD
=======
    // FIXME: update lib.rs to pass following tests

    // 1. run some (about 5) "test", show these threads are running concurrently

    // 2. run "stack", create a huge stack, handle page fault properly

>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    let mut test_num = 0;

    loop {
        print!("[>] ");
        let line = input::get_line();
        match line.trim() {
            "exit" => break,
            "ps" => {
                ysos::proc::print_process_list();
            }
            "stack" => {
<<<<<<< HEAD
                ysos::stack_thread_test();
=======
                ysos::new_stack_test_thread();
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
            }
            "test" => {
                ysos::new_test_thread(format!("{}", test_num).as_str());
                test_num += 1;
            }
            _ => println!("[=] {}", line),
        }
    }

    ysos::shutdown(boot_info);
}
