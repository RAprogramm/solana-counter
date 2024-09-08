# Counter client

## Usage

```sh
A client for interacting with a Solana counter program.

Usage: counter-client --network <NETWORK> --key-type <KEY_TYPE> --key-value <KEY_VALUE> <--initialize|--increment|-
-check>

Options:
      --initialize             Initialize the counter account
  -i, --increment              Increment the counter
  -c, --check                  Check the counter value
  -n, --network <NETWORK>      Network to use (local or devnet)
      --key-type <KEY_TYPE>    Type of key (string or file)
  -k, --key-value <KEY_VALUE>  Key value (private key string or path to keypair file)
  -h, --help                   Print help
```



## Commands example

- show info about program and all available options
```sh 
counter-client --help
```
---

- initialize counter
```sh 
counter-client --key-type string --key-value 5PJPLfTEnJ5bFJgrvkxnNffitvzYDe2LVMS1wZQcT9yofn6L7kYapNC2Qu6zszoe6LC2JB1BCuTvNvFCh8Kx9sf21 --initialize --network devnet
```
---

- increment counter
```sh 
counter-client --key-type string --key-value 5PJPLfTEnJ5bFJgrvkxnNMfftvzYDe2LVMS1wZQcT9yofn6L7kYapNC2Qu6zszoe6LC2JB1BCuTvNvFCh8Kx9sf21 --increment --network devnet
```
---

- get counter
```sh 
counter-client --key-type string --key-value 5PJPLfTEnJ5bFJgrvkxnNMitvzffDe2LVMS1wZQcT9yofn6L7kYapNC2Qu6zszoe6LC2JB1BCuTvNvFCh8Kx9sf21 --check --network devnet
```

---

### Using on localhost

1. run local network
```sh
solana-test-validator
```

> [!TIP]
>
> SETUP NETWORK
>
> Create keypairs via `solana-keygen new -o ./my_key.json`
>
>```sh
>solana config set --url localhost
>```
>```sh
>solana config set --keypair ./my_key.json
>```
>
> check network `solana config get`
>
>```sh
>Config File: /home/ra/.config/solana/cli/config.yml
>RPC URL: http://localhost:8899 
>WebSocket URL: ws://localhost:8900/ (computed)
>Keypair Path: my_key.json 
>Commitment: confirmed 
>```


> [!IMPORTANT]
>
> Make sure that user account have lamports for commission!
>
> If there is not enaugh lamports make `solana airdrop 3`. 

2. Initialize counter
```sh
counter-client --network local --key-type file --key-value my_key.json --initialize 
```

3. Increment counter
```sh
counter-client --key-type file --key-value my_key.json --increment --network local
```

4. Check counter
```sh
counter-client --key-type file --key-value my_key.json --check --network local
```
