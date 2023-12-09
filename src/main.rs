mod cache;
mod complex;
mod data;
mod functions;
mod mathcmd;
mod memory;
mod modules;
mod output;
mod point;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use std::process::exit;

use crate::{
    mathcmd::mathcmd_main as mathcmd,
    modules::{calc::calc_main as calc, solve::solve_main as solve},
    output::{output_help, output_message},
};
use current_locale::current_locale;

fn detector(_page: String) {
    let page: &str = _page.as_str();
    match page {
        "solve" => solve(),
        "calc" => calc(),
        "main" | "command" => mathcmd(),
        _ => output_message("warning.module_cannot_be_find"),
    }
}

fn main() {
    rust_i18n::set_locale(current_locale().unwrap_or("en".to_owned()).as_str());
    let args: Vec<String> = std::env::args().collect();
    let mut page: &str = "command";
    for i in args.iter().skip(1) {
        let arg: &str = i;
        if arg == "--help" || arg == "-h" {
            output_help(page);
            exit(0);
        } else {
            page = arg;
        }
    }
    detector(page.to_owned());
}
