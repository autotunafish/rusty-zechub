# Rusty-ZecHub

A playground for RUST.

* Working with JSON's in RUST
* Output a random seed phrase
* Working with Zebrad
  * Extract ZEC supply using Zebrad
  * Extract size of Zebra node on disk
  * Output current mempool with details
  * View transaction details given any txid
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
* uses `jq` for JSON formatting
  * `sudo apt install jq`


![Screenshot_2025-06-21_14-10-54](https://github.com/user-attachments/assets/c2490a40-ab04-45a9-b999-e85674a740be)




