#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! lazy_static = "1.4.0"
//! ```

use lazy_static::lazy_static;
use std::fs::File;
use std::io::{Result, Write};
use std::os::fd::FromRawFd;
use std::sync::Mutex;

lazy_static! {
    static ref RAW_STDOUT: Mutex<File> = unsafe { Mutex::new(File::from_raw_fd(1)) };
}

fn print1() -> Result<()> {
    writeln!(RAW_STDOUT.lock().unwrap(), "test1")
}

fn print2() -> Result<()> {
    writeln!(RAW_STDOUT.lock().unwrap(), "test2")
}

fn main() -> Result<()> {
    print1()?;
    print2()?;
    Ok(())
}
