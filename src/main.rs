
mod calc;
mod output;
mod solve;
mod mathcmd;
use crate::mathcmd::mathcmd_main;

rust_i18n::i18n!("locales", fallback = "en");
use current_locale::current_locale;
fn main() {
    let __locale = current_locale().unwrap();
    rust_i18n::set_locale(&__locale);
    mathcmd_main();
}

