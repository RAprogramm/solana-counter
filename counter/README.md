# Accounts counter smart contract

## Usage

### Build contract

> Build
>
>```sh
>cargo build-bpf
>```

### Network setting up

#### Check config
```sh
Config File: /home/username/.config/solana/cli/config.yml
RPC URL: http://devnet-rpc.yona.network:8899 
WebSocket URL: ws://devnet-rpc.yona.network:8900/ (computed)
Keypair Path: /home/username/.config/solana/yona-devnet.json
Commitment: confirmed
```

### Deploy to network

> [!IMPORTANT]
>
> Make sure that user account have lamports for deploying!
>
> If there is not enaugh lamports check [faucet tutorial](https://telegra.ph/Yona-Network-Devnet-Guide-05-20). 

> Deploy
>
>```sh
>solana program deploy target/deploy/counter.so
>```

