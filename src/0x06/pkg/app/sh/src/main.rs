#![no_std]
#![no_main]

extern crate alloc;

use core::clone;

use alloc::string::ToString;
use alloc::vec::Vec;
use lib::string::String;
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

pub fn sh_cd(line : Vec<&str>, path : &mut String) {
    *path = String::from(line[1]);
    return;
}

pub fn sh_cat(line : Vec<&str>) {
    let file_path = line[1];
    let file = sys_open(file_path);
    let mut buf = vec![0; 0x1000];
    let size = sys_read(file, &mut buf).unwrap();
    for ch in  buf.iter() {
        print!("{}", *ch as char)
    }
    println!("Hello filesystem from <22331109>!");
    sys_close(file);
    return;
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


fn main() -> isize {
    let mut path = String::from("/");
    println!("---------------------- Shell ------------------------");
    println!("                                 type `help` for help");
    loop {
        print!("{} > ", path);
        let input = stdin().read_line();
        let line: Vec<&str> = input.trim().split(' ').collect();
        match line[0] {
            "ls" => sys_list_dir(path.as_str()),
            "cd" => sh_cd(line, &mut path),
            "cat" => sh_cat(line),
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
