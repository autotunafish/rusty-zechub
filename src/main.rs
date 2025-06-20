use bip0039::{Count, English, Mnemonic};
use curl::easy::Easy;
use dialoguer::Select;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Result, Write};
use std::process;
use std::process::{Command, Stdio};
use std::path::Path;
//use clap::{command, Arg};

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

struct NodeConnection {
    url: String,
    port: i32,
}
fn main() {
    let my_connection = NodeConnection {
        url: String::from("http://127.0.0.1"),
        port: 8232,
    };

    init(my_connection);
    display_menu().unwrap();
    //cleanup().unwrap();
}

fn init(myserver: NodeConnection) {
    println!(
        "\nAttempting to connect to Zebrad @ {}:{}\n.\n.\n.",
        myserver.url, myserver.port
    );

    let mymethod = "getinfo";
    let body_string = format!(
        "{{\"jsonrpc\": \"1.0\", \"id\":\"curltest\", \"method\": \"{}\", \"params\": []}}",
        mymethod
    );
    let mut body = body_string.as_bytes();

    let local_connection = myserver.url + ":" + &myserver.port.to_string();

    let mut easy = Easy::new();
    easy.url(&local_connection).unwrap();
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
        transfer
            .read_function(|buf| Ok(body.read(buf).unwrap_or(0)))
            .unwrap();

        // Response body
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();

        transfer.perform().expect("Could not connect!\n")
        // .. to force drop it here, so we can use easy.response_code()
    }

    let check = easy.response_code().unwrap().to_string();

    if check == "200" {
        println!("Connected!");
    } else {
        println!("No response!");
    }
}
fn display_menu() -> Result<()> {
    let mymenu = "\nRusty-Zechub";

    let opts = [
        "Display Mnemonic ",
        "Visualize Mempool",
        "Blockchain Detail",
        "Extract Supply Info",
        "Transaction Detail",
        "Block Detail",
        "Peer Details",
        "Exit",
    ];
    let index = Select::new()
        .with_prompt(mymenu)
        .items(&opts)
        .default(0)
        .interact();

    let choice = index.unwrap();

    match choice {
        0 => {
            clear_terminal_screen();
            display_mnemonic().unwrap();
        }
        1 => {
            clear_terminal_screen();
            visualize_mempool().unwrap();
        }
        2 => {
            clear_terminal_screen();
            getblockchaininfo(false);
        }
        3 => {
            clear_terminal_screen();
            deserialize().unwrap();
        }
        4 => {
            clear_terminal_screen();
            //let mytxid = "e6726f9e4f4c627cd0f2efb5cf13e0a9537c17e13290d0e0cd828a60676c6183";
            println!("Enter your txid:\n");
            let mut input: String = String::new(); // Create a string variable
            std::io::stdin() // Get the standard input stream
            .read_line(&mut input) // The read_line function reads data until it reaches a '\n' character
            .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message
        
            tx_details(&input.trim().to_string()).unwrap();
        
        }
        5 => {
            clear_terminal_screen();

            //let match_result = command!().arg( Arg::new(input.as_mut_str())).get_matches();
            //let block = match_result.get_one::<String>("block").unwrap(); //"1000";
          
            println!("Enter your block:\n");
            let mut input: String = String::new(); // Create a string variable
            std::io::stdin() // Get the standard input stream
            .read_line(&mut input) // The read_line function reads data until it reaches a '\n' character
            .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

            getblock(&input.trim().to_string()).unwrap();
        }
        6 => {
            clear_terminal_screen();
            getpeerinfo().unwrap();
        }
        7 => {
            cleanup().unwrap();
            process::exit(1);
        }
        _ => {}
    }

    //println!("You selected {}!", choice);

    Ok(())
}
fn display_mnemonic() -> Result<()> {
    let my_mnemonic: Mnemonic<English> = Mnemonic::generate(Count::Words24);
    let mn_str = my_mnemonic.into_phrase();
    println!("Phrase: {}\n", mn_str);
    display_menu().unwrap();
    Ok(())
}
fn visualize_mempool() -> Result<()> {
    let mymethod = "getrawmempool";
    let mut body =
        r#"{"jsonrpc":"1.0", "id": "curltest", "method":"getrawmempool","params":[true]}"#
            .as_bytes();

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
        transfer
            .read_function(|buf| Ok(body.read(buf).unwrap_or(0)))
            .unwrap();

        // Response body
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();

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
        let mut file = File::create("mempool_output.json").unwrap();

        // Write the JSON string to the file
        file.write_all(result.as_bytes()).unwrap();
    }

    // Open output.json with jq to make pretty
    let mut jq_child = Command::new("/usr/bin/jq")
        .arg(".result")
        .arg("mempool_output.json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("test");

    let mut stdout = jq_child.stdout.take().unwrap();
    let mut newfile = File::create("mempool_new.json").unwrap();
    let mut buffer = String::new();

    //Read jq .result output.json into a String
    stdout.read_to_string(&mut buffer).expect("test");

    println!("{}", buffer);

    // Create a new file with result
    newfile.write_all(buffer.as_bytes()).unwrap();
    println!("\n");

    /*
    let file_path = "mempool_new.json";
    let my_json: String = std::fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let my_strings: Vec<String> = serde_json::from_str(&my_json)?;

    println!("Current Mempool: {} txid's\n", my_strings.len());
    for item in &my_strings
    {
        println!("{:#?}", item);
    }
    */

    display_menu().unwrap();
    Ok(())
}
fn getblockchaininfo(no_output: bool) {
    /*

    curl -s --data-binary '{"jsonrpc": "1.0", "id":"curltest", "method": "getinfo", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:8232/

    */
    let mymethod = "getblockchaininfo";
    let mut body =
        r#"{"jsonrpc":"1.0", "id": "curltest", "method":"getblockchaininfo","params":[]}"#
            .as_bytes();

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
        transfer
            .read_function(|buf| Ok(body.read(buf).unwrap_or(0)))
            .unwrap();

        // Response body
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();

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
        .spawn()
        .expect("test");

    let mut stdout = jq_child.stdout.take().unwrap();
    let mut newfile = File::create("new.json").unwrap();
    let mut buffer = String::new();

    //Read jq .result output.json into a String
    stdout.read_to_string(&mut buffer).expect("test");

    if no_output {
    } else {
        println!("{}", buffer);
    }

    // Create a new file with result
    newfile.write_all(buffer.as_bytes()).unwrap();
    println!("\n");

    if no_output {

    } else {
        display_menu().unwrap();
    }
}
fn deserialize() -> Result<()> {

    getblockchaininfo(true);

    let file_path = "new.json";
    //let my_json = input.clone();
    //println!("{}", serde_json::to_string_pretty(&my_json).unwrap());
    //println!("test: {}",my_json);
    let my_json: String =
        std::fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let p: BlockChainInfo = serde_json::from_str(&my_json)?;

    let total_supply = p.chain_supply.chain_value;
    let transparent_supply = p.value_pools[0].chain_value;
    let sprout_supply = p.value_pools[1].chain_value;
    let sapling_supply = p.value_pools[2].chain_value;
    let orchard_supply = p.value_pools[3].chain_value;
    let lockbox_supply = p.value_pools[4].chain_value;

    println!("Size of Zebra node on disk  | {:#?} bytes", p.size_on_disk);
    println!("ZEC in the Transparent Pool | {:#?} ", transparent_supply);
    println!("ZEC in the Sprout Pool      | {:#?}", sprout_supply);
    println!("ZEC in the Sapling Pool     | {:#?}", sapling_supply);
    println!("ZEC in the Orchard Pool     | {:#?}", orchard_supply);
    println!("ZEC in the Lockbox          | {:#?}", lockbox_supply);

    let shielded_supply = total_supply - transparent_supply - lockbox_supply;

    println!("--------------------------------------------------");
    println!("Shielded Supply             | {:#?}\n", shielded_supply);
    display_menu().unwrap();
    Ok(())
}
fn tx_details(txid: &str) -> Result<()> {
    let mymethod = "getrawtransaction";
    let body_string = format!(
        "{{\"jsonrpc\": \"1.0\", \"id\":\"curltest\", \"method\": \"{}\", \"params\": [\"{}\",1]}}",
        mymethod, txid
    );
    let mut body = body_string.as_bytes();

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
        transfer
            .read_function(|buf| Ok(body.read(buf).unwrap_or(0)))
            .unwrap();

        // Response body
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();

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
        let mut file = File::create("txid_output.json").unwrap();

        // Write the JSON string to the file
        file.write_all(result.as_bytes()).unwrap();
    }

    // Open output.json with jq to make pretty
    let mut jq_child = Command::new("/usr/bin/jq")
        .arg(".result")
        .arg("txid_output.json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("test");

    let mut stdout = jq_child.stdout.take().unwrap();
    let mut newfile = File::create("txid_new.json").unwrap();
    let mut buffer = String::new();

    //Read jq .result output.json into a String
    stdout.read_to_string(&mut buffer).expect("test");

    println!("{}", buffer);

    // Create a new file with result
    newfile.write_all(buffer.as_bytes()).unwrap();
    println!("\n");
    display_menu().unwrap();
    Ok(())
}
fn getblock(block: &str) -> Result<()> {

    
    let mymethod = "getblock";
    let body_string = format!(
        "{{\"jsonrpc\": \"1.0\", \"id\":\"curltest\", \"method\": \"{}\", \"params\": [\"{}\",2]}}",
        mymethod, block
    );

    println!("in function: {}", body_string);
    let mut body = body_string.as_bytes();

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
        transfer
            .read_function(|buf| Ok(body.read(buf).unwrap_or(0)))
            .unwrap();

        // Response body
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();

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
        let mut file = File::create("block_output.json").unwrap();

        // Write the JSON string to the file
        file.write_all(result.as_bytes()).unwrap();
    }

    // Open output.json with jq to make pretty
    let mut jq_child = Command::new("/usr/bin/jq")
        .arg(".result")
        .arg("block_output.json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("test");

    let mut stdout = jq_child.stdout.take().unwrap();
    let mut newfile = File::create("block_new.json").unwrap();
    let mut buffer = String::new();

    //Read jq .result output.json into a String
    stdout.read_to_string(&mut buffer).expect("test");

    println!("{}", buffer);

    // Create a new file with result
    newfile.write_all(buffer.as_bytes()).unwrap();
    println!("\n");
    display_menu().unwrap();
    Ok(())
}
fn getpeerinfo () -> Result<()> {
    let mymethod = "getpeerinfo";
    let body_string = format!(
        "{{\"jsonrpc\": \"1.0\", \"id\":\"curltest\", \"method\": \"{}\", \"params\": []}}",
        mymethod
    );
    let mut body = body_string.as_bytes();

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
        transfer
            .read_function(|buf| Ok(body.read(buf).unwrap_or(0)))
            .unwrap();

        // Response body
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();

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
        let mut file = File::create("peer_output.json").unwrap();

        // Write the JSON string to the file
        file.write_all(result.as_bytes()).unwrap();
    }

    // Open output.json with jq to make pretty
    let mut jq_child = Command::new("/usr/bin/jq")
        .arg(".result.[].addr")
        .arg("peer_output.json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("test");

    let mut stdout = jq_child.stdout.take().unwrap();
    let mut newfile = File::create("peer_new.json").unwrap();
    let mut buffer = String::new();

    //Read jq .result output.json into a String
    stdout.read_to_string(&mut buffer).expect("test");

    println!("{}", buffer);

    // Create a new file with result
    newfile.write_all(buffer.as_bytes()).unwrap();
    println!("\n");
    
    let file_path = "peer_new.json";
    let my_json: String = std::fs::read_to_string(file_path).expect("Couldn't find or load that file.");
   
    println!("Current peer count: {:?}\n", my_json.lines().count());

    display_menu().unwrap();

    Ok(())
}
fn cleanup() -> std::io::Result<()> {

    if Path::new("output.json").exists() {
        std::fs::remove_file("output.json")?;
    }else {};

    if Path::new("new.json").exists() {
        std::fs::remove_file("new.json")?;
    }else {};

    if Path::new("mempool_new.json").exists() {
        std::fs::remove_file("mempool_new.json")?;
    }else {};

    if Path::new("mempool_output.json").exists() {
        std::fs::remove_file("mempool_output.json")?;
    }else {};

    if Path::new("txid_new.json").exists() {
        std::fs::remove_file("txid_new.json")?;
    }else {};

    if Path::new("txid_output.json").exists() {
        std::fs::remove_file("txid_output.json")?;
    }else {};

      if Path::new("block_new.json").exists() {
        std::fs::remove_file("block_new.json")?;
    }else {};

      if Path::new("block_output.json").exists() {
        std::fs::remove_file("block_output.json")?;
    }else {};
      if Path::new("peer_output.json").exists() {
        std::fs::remove_file("peer_output.json")?;
    }else {};
      if Path::new("peer_new.json").exists() {
        std::fs::remove_file("peer_new.json")?;
    }else {};

    Ok(())
}
fn clear_terminal_screen() {
    clearscreen::clear().unwrap();
}
