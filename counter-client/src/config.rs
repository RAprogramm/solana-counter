use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair};
use std::str::FromStr;

pub enum Network {
    Local,
    Devnet,
}

pub enum KeyType {
    PrivateKey,
    KeypairFile,
}

pub struct Config {
    #[allow(dead_code)]
    pub network: Network,
    pub rpc_url: String,
    pub program_id: Pubkey,
    pub user_keypair: Keypair,
}

impl Config {
    pub fn new(network: Network, key_type: KeyType, key_value: &str) -> Self {
        let rpc_url = match network {
            Network::Local => String::from("http://localhost:8899"),
            Network::Devnet => String::from("http://devnet-rpc.yona.network:8899"),
        };

        let user_keypair = match key_type {
            KeyType::PrivateKey => Keypair::from_base58_string(key_value),
            KeyType::KeypairFile => {
                read_keypair_file(key_value).expect("Unable to read keypair file")
            }
        };

        let program_id = Pubkey::from_str("5SabfUd1J93VGgS7Tebmff4ssWu4eDrvrqspdExCucJu")
            .expect("Invalid program ID");

        Self {
            network,
            rpc_url,
            program_id,
            user_keypair,
        }
    }

    pub fn rpc_client(&self) -> RpcClient {
        RpcClient::new(&self.rpc_url)
    }
}
