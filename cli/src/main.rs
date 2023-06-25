use clap::Parser;
use cli::Loliac;

mod cli;
mod config;
mod file;

fn main() {
    let loliac = Loliac::parse();

    println!("Got {:#?}", loliac);
}
