mod functions;
mod input;
mod libs;
mod mathcmd;
mod modules;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use std::env;

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
    /// Solve mathematical equations
    Solve,
    /// Geometry calculations
    Geo,
}

fn main() -> Result<(), ()> {
    env::set_var("mathcmd_page", "main");
    rust_i18n::set_locale(current_locale().unwrap_or("en".to_owned()).as_str());
    let args: Args = Args::parse();
    if let Some(modules) = args.modules.as_ref() {
        match modules {
            Modules::Calc => calc(),
            Modules::Solve => solve(),
            Modules::Geo => geo(),
        }
    } else {
        mathcmd();
    }
    Ok(())
}
