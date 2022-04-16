#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

// use std::time::Instant;

use serde_this_or_that::*;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    // TODO
    trace!("Hello world!");

    Ok(())
}
