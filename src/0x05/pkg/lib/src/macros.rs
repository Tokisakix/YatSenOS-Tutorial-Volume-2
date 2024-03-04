use crate::alloc::string::ToString;
use crate::errln;

#[macro_export]
macro_rules! entry {
    ($fn:ident) => {
        #[export_name = "_start"]
        pub extern "C" fn __impl_start() {
            let ret = $fn();
<<<<<<< HEAD
            lib::sys_exit(ret);
=======
            // FIXME: after syscall, add lib::sys_exit(ret);
            loop {}
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
        }
    };
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let location = if let Some(location) = info.location() {
        alloc::format!(
<<<<<<< HEAD
            "{}@{}:{}",
=======
            "{}:{}:{}",
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
            location.file(),
            location.line(),
            location.column()
        )
    } else {
        "Unknown location".to_string()
    };
    let msg = if let Some(msg) = info.message() {
        alloc::format!("{}", msg)
    } else {
        "No more message...".to_string()
    };
    errln!("\n\n\rERROR: panicked at {}\n\n\r{}", location, msg);

<<<<<<< HEAD
    crate::sys_exit(1);
=======
    // FIXME: after syscall, add lib::sys_exit(1);
    loop {}
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
}
