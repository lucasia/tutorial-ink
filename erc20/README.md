# Example erc20 smart contract in ink! 

## Running Unit Tests
```
cargo test
```

## Build the contract
```
cargo contract build
```

## Upload and Instantiate the contract
### Start the local blockchain node, e.g.:
```
cd artifacts/substrate-contracts-node-linux
./substrate-contracts-node
```

### Run the following.  **COPY** the contract address for later use.
```
cargo contract instantiate --constructor new --args 1_000_000 --suri //Alice
```


## Verify the contract
### Save the contract address as a local env variable
```
export INSTANTIATED_CONTRACT_ADDRESS=<<CONTRACT_ADDRESS>>
echo $INSTANTIATED_CONTRACT_ADDRESS 
```

### Verify the total_supply() method
```
cargo contract call --contract $INSTANTIATED_CONTRACT_ADDRESS \
    --message total_supply --suri //Alice --dry-run
```

### Verify the balance_of() method
```
cargo contract call --contract $INSTANTIATED_CONTRACT_ADDRESS \
--message balance_of --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
--suri //Alice --dry-run
```

