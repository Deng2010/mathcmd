mod calc;
mod output;
mod solve;
mod mathcmd;
rust_i18n::i18n!("locales", fallback = "en");
use current_locale::current_locale;
use crate::mathcmd::mathcmd_main;
fn main() {
    let __locale = current_locale().unwrap();
    rust_i18n::set_locale(&__locale);
    mathcmd_main();
}

