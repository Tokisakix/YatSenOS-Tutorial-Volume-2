#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use alloc::vec::Vec;
use lib::*;

extern crate lib;

pub fn sh_exec(line : Vec<&str>) {
    if line.len() < 2 {
        println!("Usage: exec <app_name>");
        return;
    }

    let name = line[1];

    let pid = sys_spawn(name.to_ascii_lowercase().as_str());

    if pid == 0 {
        errln!("failed to spawn process: {}", name);
        return;
    }

    let ret = sys_wait_pid(pid);

    println!(
        "[+] process {} exited with code {}",
        pid,
        ret
    );
}

pub fn sh_kill(line : Vec<&str>) {
    if line.len() < 2 {
        println!("Usage: kill <pid>");
        return;
    }
    let pid = line[1].to_string().parse::<u16>();

    if pid.is_err() {
        errln!("Cannot parse pid");
        return;
    }

    sys_kill(pid.unwrap());
}

const HELP_INFO: &'static str = {
r#"Shell by Tokisakix
Usage:
    help            | show shell usage
    ps              | show process info
    lsapp           | show app info
    exec <app_name> | execute app
    kill <pid>      | kill process
    clear           | clear screen
    exit            | exit shell
"#
};


fn main() -> usize {
    println!("---------------------- Shell ------------------------");
    println!("                                 type `help` for help");
    loop {
        print!("> ");
        let input = stdin().read_line();
        let line: Vec<&str> = input.trim().split(' ').collect();
        match line[0] {
            "exit" => break,
            "ps" => sys_stat(),
            "lsapp" => sys_list_app(),
            "exec" => sh_exec(line),
            "kill" => sh_kill(line),
            "help" => print!("{}", HELP_INFO),
            "clear" => print!("\x1b[1;1H\x1b[2J"),
            _ => continue,
        }
    }

    0
}

entry!(main);
