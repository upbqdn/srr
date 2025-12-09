# 🦓's Scanning Results Reader

> Notice: the repo is unmaintained.

`srr` (pronounced sir) is a trivial utility for displaying Zebra's scanning
results.

## How It Works

1. Opens Zebra's scanning storage and reads the results containing scanning keys
   and TXIDs.
2. Fetches the transactions by their TXIDs from Zebra using the
   `getrawtransaction` RPC.
3. Decrypts the tx outputs using the corresponding scanning key.
4. Prints the memos in the outputs.

## How to Try It

### Scan the Block Chain with Zebra

1. Checkout this branch:
   https://github.com/ZcashFoundation/zebra/tree/tmp-fixes-do-not-merge
2. Add a viewing key to your Zebra config file:

   ``` toml
   [shielded_scan.sapling_keys_to_scan]
   "zxviews1q0duytgcqqqqpqre26wkl45gvwwwd706xw608hucmvfalr759ejwf7qshjf5r9aa7323zulvz6plhttp5mltqcgs9t039cx2d09mgq05ts63n8u35hyv6h9nc9ctqqtue2u7cer2mqegunuulq2luhq3ywjcz35yyljewa4mgkgjzyfwh6fr6jd0dzd44ghk0nxdv2hnv4j5nxfwv24rwdmgllhe0p8568sgqt9ckt02v2kxf5ahtql6s0ltjpkckw8gtymxtxuu9gcr0swvz" = 1
   ```
   This key is from [ZECpages](https://zecpages.com/boardinfo).

3. Make sure Zebra runs on Mainnet and listens on the default RPC port by having
   the following in the same config file:

    ``` toml
    [network]
    network = 'Mainnet'

    [rpc]
    listen_addr = "127.0.0.1:8232"
    ```

4. Compile and run Zebra with `--features "shielded-scan"` and your config file.

### Run `srr`

5. Place this repository besides the repository with Zebra so that the paths
   specified in
   [`Cargo.toml`](https://github.com/upbqdn/srr/blob/main/Cargo.toml) correctly
   point to Zebra's crates.
6. Compile and run `srr` by, for example, `cargo r`. 
