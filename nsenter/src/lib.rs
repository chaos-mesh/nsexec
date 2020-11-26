use nix::fcntl::open;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;

use std::path::PathBuf;

#[ctor::ctor]
#[no_mangle]
unsafe fn nsenter() {
    let pid = std::env::var("__MNTEXEC_PID").unwrap();
    let pid: i32 = pid.parse().unwrap();

    let fd = open(&PathBuf::from(format!("/proc/{}/ns/mnt", pid)), OFlag::O_RDONLY, Mode::empty()).unwrap();
    let ret = libc::setns(fd, libc::CLONE_NEWNS);

    println!("SET MNT NS {}", ret);
}