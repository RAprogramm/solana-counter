# Accounts counter

- [Client](./counter-client/README.md)
- [Smart Contract](./counter/README.md)

## How to get `keypair.json` from `private_key_long_string` with python

> [!TIP]
>
> Private key you can get in Yona Wallet.
> 
> About `Yona Wallet` or `Phantom Wallet`
>
> [1. Yona Wallet Tutorial](https://telegra.ph/Yona-Network-Devnet-Guide-05-20)
>
> [2. Download zip with chrome extention](https://yona.network/yona_wallet.zip)

 
1. [Private key decoder to JSON file](./decoder/README.md)

2. Run script with your private key.

3. Add your private key to solana cli config.

> [!TIP]
>
>```sh
>solana config set --keypair <path/to/json/file/generated/by/decoder> 
>```


