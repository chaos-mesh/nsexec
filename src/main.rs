use structopt::StructOpt;

use std::process::Command;

#[derive(StructOpt, Debug)]
#[structopt(
    setting = structopt::clap::AppSettings::TrailingVarArg
)]
struct Opt {
    #[structopt(short, long)]
    pid: i32,

    #[structopt(short, long, default_value="/lib/libnsenter.so")]
    library_path: String,

    #[structopt(required = true)]
    pub extra: Vec<String>
}

fn main() {
    let opts = Opt::from_args();

    let mut command = Command::new(&opts.extra[0]);
    if opts.extra.len() > 1 {
        command.args(&opts.extra[1..]);
    }

    command.env("LD_PRELOAD", opts.library_path);
    command.env("__MNTEXEC_PID", format!("{}", opts.pid));
    let mut child = command.spawn().unwrap();
    child.wait().unwrap();
}
