#![no_std]

<<<<<<< HEAD
use num_enum::TryFromPrimitive;
=======
use num_enum::FromPrimitive;
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

pub mod macros;

#[repr(usize)]
<<<<<<< HEAD
#[derive(Clone, Debug, TryFromPrimitive)]
=======
#[derive(Clone, Debug, FromPrimitive)]
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
pub enum Syscall {
    Read = 0,
    Write = 1,

    GetPid = 39,
<<<<<<< HEAD

    Spawn = 59,
    Exit = 60,
    WaitPid = 61,
    Kill = 62,

    Time = 201,

    ListApp = 65529,
    Stat = 65530,
=======
    
    Spawn = 59,
    Exit = 60,
    WaitPid = 61,

    ListApp = 65531,
    Stat = 65532,
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
    Allocate = 65533,
    Deallocate = 65534,

    #[num_enum(default)]
<<<<<<< HEAD
    None = 65535,
=======
    Unknown = 65535,
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
}
