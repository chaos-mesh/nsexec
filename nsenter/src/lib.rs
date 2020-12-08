use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::sched::{setns, CloneFlags};

use std::path::PathBuf;

#[ctor::ctor]
#[no_mangle]
unsafe fn nsenter() {
    if let Ok(ns_path) = std::env::var("__MNTEXEC_PATH") {
        let fd = open(&PathBuf::from(ns_path), OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWNS).unwrap();
    }
}