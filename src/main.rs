use serde::{Deserialize, Serialize};
use std::fs::{File};
use bip0039::{Count, English, Mnemonic};
use std::io::{Result,Read,Write};
use curl::easy::{Easy};
use std::process::{Command,Stdio};


#[derive(Serialize, Deserialize, Debug)]
struct BlockChainInfo {
    chain: String,
    blocks: i64,
    size_on_disk: i64,
    #[serde(alias = "chainSupply")]
    chain_supply: SupplyInfo,
    #[serde(alias = "valuePools")]
    value_pools: Vec<NodeData>,
}
#[derive(Serialize, Deserialize, Debug)]
struct SupplyInfo {
    #[serde(alias = "chainValue")]
    chain_value: f64,
    monitored: bool,
}
#[derive(Serialize, Deserialize, Debug)]
struct NodeData {
    id: String,
    #[serde(alias = "chainValue")]
    chain_value: f64,
    monitored: bool,
}

fn main()  {
    
    println!("\nHello, world!\n\n");
    display_mnemonic().unwrap();
    to_curl();
    deserialize().unwrap();
    visualize_mempool().unwrap();
    cleanup().unwrap();
     
}

fn display_mnemonic() -> Result<()> {

        let my_mnemonic:Mnemonic<English> = Mnemonic::generate(Count::Words24);
        let mn_str = my_mnemonic.into_phrase();
        println!("Phrase: {}\n", mn_str);
        Ok(())
}
fn deserialize() -> Result<()> {


    let file_path = "new.json";
    //let my_json = input.clone();
    //println!("{}", serde_json::to_string_pretty(&my_json).unwrap());
    //println!("test: {}",my_json);
    let my_json: String = std::fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let p: BlockChainInfo = serde_json::from_str(&my_json)?;
    
    let total_supply = p.chain_supply.chain_value;
    let transparent_supply = p.value_pools[0].chain_value;
    let sprout_supply = p.value_pools[1].chain_value;
    let sapling_supply = p.value_pools[2].chain_value;
    let orchard_supply = p.value_pools[3].chain_value;
    let lockbox_supply = p.value_pools[4].chain_value;

    println!("Size of Zebra node on disk  | {:#?} bytes",p.size_on_disk);
    println!("ZEC in the Transparent Pool | {:#?} ",transparent_supply);
    println!("ZEC in the Sprout Pool      | {:#?}",sprout_supply);
    println!("ZEC in the Sapling Pool     | {:#?}",sapling_supply);
    println!("ZEC in the Orchard Pool     | {:#?}",orchard_supply);
    println!("ZEC in the Lockbox          | {:#?}",lockbox_supply);


    let shielded_supply = total_supply -transparent_supply;

    println!("--------------------------------------------------");
    println!("Shielded Supply             | {:#?}\n", shielded_supply);
    
    Ok(())

}
fn visualize_mempool() -> Result<()> {
    
    let file_path = "memtest.json";
    let my_json: String = std::fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let my_strings: Vec<String> = serde_json::from_str(&my_json)?;

    println!("Current Mempool: {} txid's\n", my_strings.len());
    for item in &my_strings 
    {
        println!("{:?}", item);
    }
    
    Ok(())
}
fn to_curl() {

    /* 
    
    curl -s --data-binary '{"jsonrpc": "1.0", "id":"curltest", "method": "getinfo", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:8232/
    
    */
    let mymethod = "getblockchaininfo";
    let mut body = r#"{"jsonrpc":"1.0", "id": "curltest", "method":"getblockchaininfo","params":[]}"#.as_bytes();

    let mut easy = Easy::new();
    easy.url("http://127.0.0.1:8232").unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(body.len() as u64).unwrap();

    // Set the Content-Type header to application/json
    let mut list = curl::easy::List::new();
    list.append("Content-Type: application/json").unwrap();
    easy.http_headers(list).unwrap();


    let mut data = Vec::new();
    {
        // Create transfer in separate scope ...
        let mut transfer = easy.transfer();

        // Request body
        transfer.read_function(|buf| {
            Ok(body.read(buf).unwrap_or(0))
        }).unwrap();

        // Response body
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();

        transfer.perform().unwrap();
        // .. to force drop it here, so we can use easy.response_code()

    }

    println!("Zebrad RPC    : {:#?}", mymethod);
    println!("Response      :  {}", easy.response_code().unwrap());
    println!("Received bytes:  {} \n", data.len());
   
  
    if !data.is_empty() {
        //println!("Bytes: {:?}", data);
        //println!("As JSON: {}\n", String::from_utf8_lossy(&data));

        let result = String::from_utf8_lossy(&data);
    
    
        // Create a file to write to. Replace "output.json" with your desired file name.
        let mut file = File::create("output.json").unwrap();

        // Write the JSON string to the file
        file.write_all(result.as_bytes()).unwrap();

    }


    // Open output.json with jq to make pretty
    let mut jq_child = Command::new("/usr/bin/jq")
    .arg(".result")
    .arg("output.json")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn().expect("test");


    let mut stdout = jq_child.stdout.take().unwrap();
    let mut newfile = File::create("new.json").unwrap();
    let mut buffer = String::new();

    //Read jq .result output.json into a String
    stdout.read_to_string(&mut buffer).expect("test");

    println!("{}", buffer);  


    // Create a new file with result
    newfile.write_all(buffer.as_bytes()).unwrap();
    println!("\n");


}

fn cleanup() -> std::io::Result<()> {

    std::fs::remove_file("output.json")?;
    std::fs::remove_file("new.json")?;

    Ok(())

}
