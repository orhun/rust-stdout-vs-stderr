#!/usr/bin/env rust-script

use std::{
    io::{self, BufWriter, Result, Write},
    time::Instant,
};

fn main() -> Result<()> {
    let first = Instant::now();
    for i in 1..1000000 {
        println!("{}", i);
    }
    let first_elapsed = first.elapsed();

    let second = Instant::now();
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout);
    for i in 1..1000000 {
        writeln!(output, "{}", i)?;
    }
    let second_elapsed = second.elapsed();
    output.flush()?;

    println!("Line buffered: {:?}", first_elapsed);
    println!("Block buffered: {:?}", second_elapsed);

    Ok(())
}
