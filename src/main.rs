pub mod api;
pub mod config;
pub mod opt;
pub mod types;

use crate::opt::Opts;
use structopt::StructOpt;

fn main() {
    let args = Opts::from_args();
    api::run(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
    });
}
