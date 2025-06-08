use simple-redis-with-rust::{DEFAULT_PORT, clients::Client};

use bytes::Bytes;
use clap::{Parser, Subcommand};
use std::num::ParseIntError;
use std::str;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(
    name = "simple-redis-with-rust",
    version,
    author,
    about = "Issue Redis commands"
)]
#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[arg(id = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Debug)]
enum Command {
    Ping {
        // message to ping
        msg: Option<Bytes>,
    },
    Get {
        // name of key
        key: String,
    },
    Set {
        // name of key to set
        key: String,
        // value to set
        val: Bytes,
        // expire the value after specified amount of time
        expires: Option<Duration>,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> simple_redis_with_rust::Result<()> {
    
}