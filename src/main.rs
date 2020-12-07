use structopt::StructOpt;

use nix::sched::{CloneFlags, setns};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;

use std::process::Command;
use std::path::PathBuf;
use std::os::unix::process::CommandExt;

#[derive(StructOpt, Debug)]
#[structopt(
    setting = structopt::clap::AppSettings::TrailingVarArg
)]
struct Opt {
    #[structopt(short, long)]
    target: i32,

    #[structopt(short, long)]
    cgroup: bool,

    #[structopt(short, long)]
    ipc: bool,

    #[structopt(short, long)]
    mnt: bool,

    #[structopt(short, long)]
    local: bool,

    #[structopt(short, long)]
    net: bool,

    #[structopt(short, long)]
    pid: bool,

    // TODO: support user
    // TODO: support uts

    #[structopt(short, long, default_value="/usr/local/lib/libnsenter.so")]
    library_path: String,

    #[structopt(required = true)]
    pub cmd: Vec<String>
}

fn main() {
    let opts = Opt::from_args();

    if opts.cgroup {
        let fd = open(&PathBuf::from(format!("/proc/{}/ns/cgroup", opts.target)), OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWCGROUP).unwrap();
    }

    if opts.ipc {
        let fd = open(&PathBuf::from(format!("/proc/{}/ns/ipc", opts.target)), OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWIPC).unwrap();
    }

    if opts.net {
        let fd = open(&PathBuf::from(format!("/proc/{}/ns/net", opts.target)), OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWNET).unwrap();
    }

    if opts.pid {
        let fd = open(&PathBuf::from(format!("/proc/{}/ns/pid", opts.target)), OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWPID).unwrap();
    }

    let mut command = if opts.mnt && opts.local {
        let mut command = Command::new("/lib64/ld-linux-x86-64.so.2");
    
        let cmd = opts.cmd.iter().map(|s| s.as_str());
        let args: Vec<&str> = ["--preload", &opts.library_path].iter().map(|s| *s).chain(cmd).collect();
        command.args(args);

        command.env("__MNTEXEC_PID", format!("{}", opts.target));

        command
    } else {
        if opts.mnt {
            let fd = open(&PathBuf::from(format!("/proc/{}/ns/mnt", opts.target)), OFlag::O_RDONLY, Mode::empty()).unwrap();
            setns(fd, CloneFlags::CLONE_NEWNS).unwrap();
        }

        let mut command = Command::new(&opts.cmd[0]);
        if opts.cmd.len() > 1 {
            command.args(&opts.cmd[1..]);
        }

        command
    };

    // TODO: set uid and gid through arguments
    command.uid(0);
    command.gid(0);
    let mut child = command.spawn().unwrap();
    child.wait().unwrap();
}
