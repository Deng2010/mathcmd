mod calc;
mod output;
mod solve;
mod mathcmd;
mod functions;
mod data;
mod memory_calc;
#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");
use current_locale::current_locale;
use crate::mathcmd::mathcmd_main;

fn main() {
    let __locale = current_locale().unwrap_or("en".to_string());
    rust_i18n::set_locale(&__locale);
    mathcmd_main();
}

