mod functions;
mod input;
mod libs;
mod mathcmd;
mod modules;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use crate::{
    mathcmd::mathcmd_main as mathcmd,
    modules::{calc::calc_main as calc, geo::geo_main as geo, solve::solve_main as solve},
};
use current_locale::current_locale;

use clap::{Parser, Subcommand};
/// A simple program to solve math problems
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Choose modules
    #[command(subcommand)]
    modules: Option<Modules>,
}
#[derive(Subcommand)]
enum Modules {
    /// Basic calculations
    Calc,
    /// Solve Mathematical equations
    Solve,
    /// Geometry Calculations
    Geo,
}
fn main() {
    let args: Args = Args::parse();
    if args.modules.is_some() {
        let modules: &Modules = args.modules.as_ref().unwrap();
        match modules {
            Modules::Calc => calc(),
            Modules::Solve => solve(),
            Modules::Geo => geo(),
        }
    } else {
        mathcmd()
    }
    rust_i18n::set_locale(current_locale().unwrap_or("en".to_owned()).as_str());
}
