mod cache;
mod calc;
mod data;
mod functions;
mod mathcmd;
mod memory;
mod modules;
mod output;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use crate::mathcmd::mathcmd_main;
use current_locale::current_locale;

fn main() {
    rust_i18n::set_locale(&current_locale().unwrap_or("en".to_string()));
    mathcmd_main();
}
