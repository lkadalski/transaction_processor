use anyhow::Result;
use std::{
    env, fs,
    io::{self, BufRead, BufReader},
};
fn main() -> Result<()> {
    let input = env::args().nth(1);
    if let Err(err) = handler::input::handle(input) {
        eprintln!("Error occured during processing: {err}");
        std::process::exit(1)
    }
}
