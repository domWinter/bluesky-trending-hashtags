use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
#[clap(version="1.0")]
#[derive(Debug, Default)]
pub struct Config {
    #[clap(long, env="SERVER_PORT", default_value = "8080")]
    pub server_port: u16,

    #[clap(long, env="SERVER_ADDR", default_value = "127.0.0.1")]
    pub server_addr: String,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::parse();
}