mod mach;
mod process;
use nix::unistd::Pid;
use std::error::Error;
use std::ffi::CString;
fn main() -> Result<(), Box<dyn Error>> {
    let this = unsafe { libc::getpid() };
    println!("this pid : {}", this);
    let pid = process::execute(
        CString::new("./a.out")?,
        &[CString::new("")?],
        &[CString::new("")?],
    )?;
    println!("child pid : {}", pid);
    mach::test_thread_for_pid(Pid::from_raw(pid));
    let mut mach_control = mach::MachProcess::new(Pid::from_raw(pid));
    loop {
        let msg = mach_control.mach_msg_receive();
        match msg {
            Some(sig) => println!("Received signal {}", sig.as_str()),
            None => println!("no message"),
        }
        break;
    }
    println!("MESSAGE RECEIVED!");
    Ok(())
}
