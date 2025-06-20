# Rusty-ZecHub

A playground for RUST.

* Working with JSON's in RUST
* Output a random seed phrase
* Working with Zebrad
  * Extract ZEC supply using Zebrad
  * Extract size of Zebra node on disk
  * Output current mempool with details
  * View transaction details given any txid

`git clone https://github.com/dismad/rusty-zechub.git`

then

```bash
cd rusty-zechub
cargo run
```
***note***

* You will need a fully synced *zebrad* reachable via `http://127.0.0.1:8232`
* uses `jq` for JSON formatting
  * `sudo apt install jq`

![Screenshot_2025-06-17_10-18-01](https://github.com/user-attachments/assets/7698df7d-5efc-4c19-b795-804ac3f56165)




