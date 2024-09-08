use clap::{ArgGroup, Parser};

#[derive(Debug, Parser)]
#[command(
    name = "Counter Client",
    about = "A client for interacting with a Solana counter program."
)]
#[command(group(
    ArgGroup::new("action")
        .required(true)
        .args(&["initialize", "increment", "check"]),
))]
pub struct Cli {
    #[arg(long, help = "Initialize the counter account")]
    pub initialize: bool,
    #[arg(long, short, help = "Increment the counter")]
    pub increment: bool,
    #[arg(long, short, help = "Check the counter value")]
    pub check: bool,
    #[arg(
        long,
        short,
        help = "Network to use (local or devnet)",
        required = true
    )]
    pub network: String,
    #[arg(long, help = "Type of key (string or file)", required = true)]
    pub key_type: String,
    #[arg(
        long,
        short,
        help = "Key value (private key string or path to keypair file)",
        required = true
    )]
    pub key_value: String,
}
