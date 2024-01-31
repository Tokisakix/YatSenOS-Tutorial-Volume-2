#![no_std]
#![no_main]

#[macro_use]
extern crate log;

use core::arch::asm;
use ysos_kernel as ysos;

boot::entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static boot::BootInfo) -> ! {
    ysos::init(boot_info);

    loop {
        ysos::print!("> ");
        let input = ysos::input::get_line();

        match input.trim() {
            "exit" => break,
            _ => {
                ysos::println!("You said: {}", input);
                ysos::println!("The counter value is {}", ysos::interrupt::clock::read_counter());
            }
        }
    }

    ysos::shutdown(boot_info);
}
