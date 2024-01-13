mod functions;
mod input;
mod libs;
mod mathcmd;
mod modules;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use std::{collections::VecDeque, process::exit};

use crate::{
    libs::output::{output_help, output_message},
    mathcmd::mathcmd_main as mathcmd,
    modules::{calc::calc_main as calc, geo::geo_main as geo, solve::solve_main as solve},
};
use current_locale::current_locale;

static VERSION: &str = "0.3.2";

fn detector(_page: String) {
    let page: &str = _page.as_str();
    match page {
        "solve" => solve(),
        "calc" => calc(),
        "geo" => geo(),
        "main" | "command" => mathcmd(),
        _ => output_message("warning.module_cannot_be_find"),
    }
}

fn main() {
    rust_i18n::set_locale(current_locale().unwrap_or("en".to_owned()).as_str());
    let mut args: VecDeque<String> = std::env::args().collect();
    args.pop_front();
    let mut page: &str = "command";
    for i in args.iter() {
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
