use crossbeam_queue::ArrayQueue;
use alloc::string::String;
use pc_keyboard::DecodedKey;
use x86_64::instructions::interrupts;
use crate::drivers::serial;

const INPUT_BUFFER_SIZE: usize = 128;

once_mutex!(pub INPUT_BUFFER: ArrayQueue<DecodedKey>);
guard_access_fn!(pub get_input_buffer(INPUT_BUFFER: ArrayQueue<DecodedKey>));

pub fn init() {
    init_INPUT_BUFFER(ArrayQueue::<DecodedKey>::new(INPUT_BUFFER_SIZE));
    info!("Input Initialized.");
}

pub fn push_key(key: DecodedKey) {
    if let Some(queue) = get_input_buffer(){
        if queue.push(key).is_err() {
            warn!("Input buffer is full. Dropping key '{:?}'", key);
        }
    }
}

pub fn try_get_key() -> Option<DecodedKey> {
    interrupts::without_interrupts(|| get_input_buffer_for_sure().pop())
}

pub fn get_key() -> DecodedKey {
    loop {
        if let Some(key) = try_get_key() {
            return key;
        }
    }
}

pub fn get_line() -> String {
    let mut s = String::with_capacity(INPUT_BUFFER_SIZE);
    loop {
        let key = get_key();
        if let DecodedKey::Unicode(k) = key {
            match k {
                '\n' => break,
                '\x08' => {
                    if !s.is_empty() {
                        serial::backspace();
                        s.pop();
                    }
                }
                c => {
                    print!("{}", k);
                    s.push(c)
                }
            }
        }
    }
    println!();
    s
}