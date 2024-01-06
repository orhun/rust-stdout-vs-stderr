#!/usr/bin/env rust-script

use std::fs::File;
use std::io::{Result, Write};
use std::os::fd::FromRawFd;

fn print1() -> Result<()> {
    let mut raw_stdout = unsafe { File::from_raw_fd(1) };
    writeln!(raw_stdout, "test1")
}

fn print2() -> Result<()> {
    let mut raw_stdout = unsafe { File::from_raw_fd(1) };
    writeln!(raw_stdout, "test2")
}

fn main() -> Result<()> {
    print1()?;
    print2()?;
    Ok(())
}
