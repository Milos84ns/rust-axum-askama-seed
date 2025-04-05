use std::env;
use lazy_static::lazy_static;

pub static PORT: u16 = 11706;

lazy_static! {
    pub static ref ENV: String = env::var("ENV").unwrap_or_else(|_| "local".to_owned());
    pub static ref APP: String =
        env::var("COMPONENT").unwrap_or_else(|_| String::from("Rust-Askama-Seed"));
    pub static ref GROUP: String = env::var("GROUP").unwrap_or_else(|_| String::from("tool"));
    pub static ref HOST: String = env::var("HOST").unwrap_or_else(|_| "local".to_owned());
    pub static ref APP_ROOT: String = env::var("APPS").unwrap_or_else(|_| String::from("unknown"));
    pub static ref USER: String = env::var("USER").unwrap_or_else(|_| String::from("unknown"));
    pub static ref DATA_CENTER: String =
        env::var("DATA_CENTRE").unwrap_or_else(|_| "sd16".to_owned());
    pub static ref ZONE: String = env::var("ZONE").unwrap_or_else(|_| "DRN".to_owned());
    pub static ref COUNTRY: String = env::var("COUNTRY").unwrap_or_else(|_| "srb".to_owned());
    pub static ref DNS_ALIAS: String = env::var("DNS_ALIAS").unwrap_or_default();
    pub static ref LOCAL_MODE: String =
        env::var("LOCAL_MODE").unwrap_or_else(|_| "false".to_owned());
    pub static ref VERSION: String =
        env::var("VERSION").unwrap_or_else(|_| env!("CARGO_PKG_VERSION").parse().unwrap());
}