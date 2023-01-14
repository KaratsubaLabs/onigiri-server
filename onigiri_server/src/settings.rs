use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SETTINGS_NO_API_KEY: bool = env::var("ONIGIRI_NO_API_KEY").is_ok();
}
