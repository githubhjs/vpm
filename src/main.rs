mod cmd;
mod config;

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

use crate::cmd::{Cmd, Run};
use crate::error::SilentExit;

#[tokio::main]
async fn main() -> ExitCode {
    // Forcibly disable backtraces
    env::remove_var("RUST_LIB_BACKTRACE");
    env::remove_var("RUST_BACKTRACE");

    match Cmd::parse().run() {
        Ok(()) => ExitCode:SUCCESS,
        Err(e) => match e.downcast::<SilentExit>() {
            Ok(SilentExit { code }) => code.into(),
            Err(e) => {
                _ = writeln!(io::stderr, "VPM: {e:?}");
                ExitCode::FAILURE
            }
        }
    }
}
