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
    cgroup: Option<PathBuf>,

    #[structopt(short, long)]
    ipc: Option<PathBuf>,

    #[structopt(short, long)]
    mnt: Option<PathBuf>,

    #[structopt(short, long)]
    net: Option<PathBuf>,

    #[structopt(short, long)]
    pid: Option<PathBuf>,

    #[structopt(short, long)]
    local: bool,

    // TODO: support user
    // TODO: support uts

    #[structopt(long, default_value="/usr/local/lib/libnsenter.so")]
    library_path: String,

    #[structopt(required = true)]
    pub cmd: Vec<String>
}

fn main() {
    let opts = Opt::from_args();

    if let Some(cgroup) = opts.cgroup {
        let fd = open(&cgroup, OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWCGROUP).unwrap();
    }

    if let Some(ipc) = opts.ipc {
        let fd = open(&ipc, OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWIPC).unwrap();
    }

    if let Some(net) = opts.net {
        let fd = open(&net, OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWNET).unwrap();
    }

    if let Some(pid) = opts.pid {
        let fd = open(&pid, OFlag::O_RDONLY, Mode::empty()).unwrap();
        setns(fd, CloneFlags::CLONE_NEWPID).unwrap();
    }

    let mut command = if let Some(mnt) = opts.mnt {
        let mut command = Command::new(&opts.cmd[0]);
        if opts.cmd.len() > 1 {
            command.args(&opts.cmd[1..]);
        }

        if opts.local {
            command.env("LD_PRELOAD", &opts.library_path);
            command.env("__MNTEXEC_PATH", format!("{}", mnt.display()));
        } else {
            let fd = open(&mnt, OFlag::O_RDONLY, Mode::empty()).unwrap();
            setns(fd, CloneFlags::CLONE_NEWNS).unwrap();
        }

        command
    } else {
        let mut command = Command::new(&opts.cmd[0]);
        if opts.cmd.len() > 1 {
            command.args(&opts.cmd[1..]);
        }

        command
    };
    
    let mut child = command.spawn().unwrap();
    child.wait().unwrap();
}
