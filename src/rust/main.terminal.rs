#![feature(slice_patterns)]

extern crate rand;
extern crate term_size;
extern crate regex;
extern crate clap;
extern crate serde_json;
extern crate base64;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate enum_primitive;

#[macro_use]
extern crate serde_derive;

use std::process;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};

mod buffer;
mod frame;
mod instruction;
mod quetzal;
mod traits;
mod ui_terminal;
mod options;
mod zmachine;

use traits::UI;
use ui_terminal::TerminalUI;
use options::Options;
use zmachine::Zmachine;


const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn main() {
    let matches = App::new("encrusted")
        .version(VERSION)
        .about("A zmachine interpreter")
        .arg(Arg::with_name("FILE")
            .help("Sets the story file to run")
            .required(true))
        .get_matches();

    let path = Path::new(matches.value_of("FILE").unwrap());

    if !path.is_file() {
        println!("\nCouldn't find game file: \n   {}\n", path.to_string_lossy());
        process::exit(1);
    }

    let mut data = Vec::new();
    let mut file = File::open(path).expect("Error opening file");
    file.read_to_end(&mut data).expect("Error reading file");

    let version = data[0];

    if version == 0 || version > 8 {
        println!("\n\
            \"{}\" has an nsupported game version: {}\n\
            Is this a valid game file?\n", path.to_string_lossy(), version);
        process::exit(1);
    }

    let ui = TerminalUI::new();
    let mut opts = Options::default();
    opts.save_dir = path.parent().unwrap().to_string_lossy().into_owned();
    opts.save_name = path.file_stem().unwrap().to_string_lossy().into_owned();

    let mut zvm = Zmachine::new(data, ui, opts);

    zvm.run();
}