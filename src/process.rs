use libc::pid_t;
use nix::libc::*;
use std::error::Error;
use std::ffi::CString;
use std::{mem::MaybeUninit, ptr};

const POSIX_SPAWN_DISABLE_ASLR: i32 = 0x0100;
const POSIX_SPAWN_FLAGS: i16 = (POSIX_SPAWN_START_SUSPENDED
    | POSIX_SPAWN_SETSIGDEF
    | POSIX_SPAWN_SETSIGMASK
    | POSIX_SPAWN_DISABLE_ASLR) as i16;

pub fn execute(
    program: CString,
    argv: &[CString],
    envar: &[CString],
) -> Result<pid_t, Box<dyn Error>> {
    let mut attr: MaybeUninit<posix_spawnattr_t> = MaybeUninit::uninit();
    let mut res = unsafe { posix_spawnattr_init(attr.as_mut_ptr()) };
    if res != 0 {
        eprintln!("Can't initialise posix_spawnattr_t");
    }
    let mut attr = unsafe { attr.assume_init() };

    res = unsafe { posix_spawnattr_setflags(&mut attr, POSIX_SPAWN_FLAGS) };
    if res != 0 {
        eprintln!("Failed to set spawn flags");
    };

    let mut args: Vec<*mut c_char> = argv.iter().map(|s| s.clone().into_raw()).collect();

    args.push(ptr::null_mut());

    let mut envs: Vec<*mut c_char> = envar.iter().map(|s| s.clone().into_raw()).collect();
    envs.push(ptr::null_mut());

    // request_trace().map_err(|e| RunError::Trace(e.to_string()))?;
    let mut pid: pid_t = 0;
    unsafe {
        posix_spawnp(
            &mut pid,
            program.into_raw(),
            ptr::null_mut(),
            &attr,
            args.as_ptr(),
            envs.as_ptr(),
        );
    }

    unsafe { posix_spawnattr_destroy(&mut attr) };

    Ok(pid)
}
