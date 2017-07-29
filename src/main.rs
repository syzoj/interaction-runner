extern crate os_pipe;
#[macro_use]
extern crate clap;

use clap::App;
use std::process::Command;
use os_pipe::{pipe, IntoStdio};

fn main() {
    let args = load_yaml!("args_en.yaml");
    let matches = App::from_yaml(args).get_matches();
    let prg1 = matches.value_of("prg1").unwrap();
    let prg2 = matches.value_of("prg2").unwrap();

    let mut cmd1 = Command::new(prg1);
    let mut cmd2 = Command::new(prg2);

    let (to2, from1) = pipe().unwrap();
    let (to1, from2) = pipe().unwrap();

    cmd1.stdin(to1.into_stdio());
    cmd2.stdin(to2.into_stdio());
    cmd1.stdout(from1.into_stdio());
    cmd2.stdout(from2.into_stdio());

    let mut handle1 = cmd1.spawn().unwrap();
    let mut handle2 = cmd2.spawn().unwrap();

    drop(cmd1);
    drop(cmd2);
    let r1 = handle1.wait().unwrap();
    let r2 = handle2.wait().unwrap();

    if matches.is_present("show_status") {
        println!("{}: {}", prg1, r1);
        println!("{}: {}", prg2, r2);
    }
}
