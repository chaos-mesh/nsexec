use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::sched::{setns, CloneFlags};

use std::path::PathBuf;

#[ctor::ctor]
#[no_mangle]
unsafe fn nsenter() {
    if let Ok(pid) = std::env::var("__MNTEXEC_PID") {
        let pid: i32 = pid.parse().unwrap();

        let fd = open(&PathBuf::from(format!("/proc/{}/ns/mnt", pid)), OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWNS).unwrap();

        std::env::remove_var("LD_PRELOAD")
    }
}