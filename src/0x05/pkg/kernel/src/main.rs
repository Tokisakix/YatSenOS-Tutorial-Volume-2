#![no_std]
#![no_main]

use ysos::*;
use ysos_kernel as ysos;

extern crate alloc;

boot::entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static boot::BootInfo) -> ! {
    ysos::init(boot_info);
    ysos::wait(spawn_init());
    ysos::shutdown(boot_info);
}

pub fn spawn_init() -> proc::ProcessId {
<<<<<<< HEAD
    // print_serial!("\x1b[1;1H\x1b[2J");
=======
    // NOTE: you may want to clear the screen before starting the shell
    // print_serial!("\x1b[1;1H\x1b[2J");

>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    proc::list_app();
    proc::spawn("sh").unwrap()
}
