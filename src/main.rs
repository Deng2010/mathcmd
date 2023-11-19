mod cache;
mod complex;
mod data;
mod functions;
mod mathcmd;
mod memory;
mod modules;
mod output;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use std::process::exit;

use crate::{
    mathcmd::mathcmd_main as mathcmd,
    modules::{calc::calc_main as calc, solve::solve_function as solve},
    output::output_help,
};
use current_locale::current_locale;

fn detector(_page: String) {
    let page: &str = _page.as_str();
    match page {
        "solve" => solve(),
        "calc" => calc(),
        _ => mathcmd(),
    }
}

fn main() {
    rust_i18n::set_locale(current_locale().unwrap_or("en".to_string()).as_str());
    let args: Vec<String> = std::env::args().collect();
    let mut page: &str = "main";
    for i in args.iter().skip(1) {
        let arg: &str = i;
        if arg == "--help" {
            output_help(page);
            exit(0);
        } else if arg.starts_with("--mod=") {
            page = arg.strip_prefix("--mod=").unwrap();
        }
    }
    detector(page.to_owned());
}
