// #![allow(non_camel)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

mod common {
    pub mod constants;
    pub mod types;
}
mod world {
    pub mod world;
}
mod hardware {
    pub mod memory;
}
mod utils;

fn main() {
    // Set up log files
    let ivory_page_log = WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("ivoryPageLog.log").unwrap(),
    );
    let run_log = WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("run.log").unwrap(),
    );

    println!("Hello, world!");
}
