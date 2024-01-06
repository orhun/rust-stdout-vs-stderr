#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! lazy_static = "1.4.0"
//! ```

use lazy_static::lazy_static;
use std::fs::File;
use std::io::{Result, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

lazy_static! {
    static ref RAW_STDOUT_FD: OwnedFd = {
        let stdout = std::io::stdout();
        let raw_fd = stdout.as_raw_fd();
        unsafe { OwnedFd::from_raw_fd(raw_fd) }
    };
}

fn print1() -> Result<()> {
    let mut raw_stdout = File::from(RAW_STDOUT_FD.try_clone()?);
    writeln!(raw_stdout, "test1")
}

fn print2() -> Result<()> {
    let mut raw_stdout = File::from(RAW_STDOUT_FD.try_clone()?);
    writeln!(raw_stdout, "test2")
}

fn main() -> Result<()> {
    print1()?;
    print2()?;
    Ok(())
}
