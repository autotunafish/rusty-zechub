# Rusty-ZecHub

A playground for RUST.

* Working with JSON's in RUST
* Output a random seed phrase
* Working with Zebrad
  * Extract ZEC supply using Zebrad
  * Extract size of Zebra node on disk
  * Output current mempool with details
  * View transaction details given any txid
  * View transaction type (Transparent/Sapling/Orchard) given any txid
  * Vuew transaction date given either a txid or block
  * View block details given any block
  * View peers connected to your node

`git clone https://github.com/dismad/rusty-zechub.git`

then

```bash
cd rusty-zechub
cargo run --release
```
***note***

* You will need a fully synced *zebrad* reachable via `http://127.0.0.1:8232`
* `cargo install --git https://github.com/ZcashFoundation/zebra zebrad`

* Edit zebrad.toml 
```
[rpc]
listen_addr = "127.0.0.1:8232"
enable_cookie_auth = false
```
* Start zebra:
  
  `zebrad start`

* uses `jq` for JSON formatting
  * `sudo apt install jq`

![Screenshot_2025-06-23_17-44-26](https://github.com/user-attachments/assets/549878b1-c39f-4632-82bc-ead26ebc9837)





