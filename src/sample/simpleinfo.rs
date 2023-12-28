#![allow(
    clippy::unreadable_literal,
    clippy::upper_case_acronyms,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    unused_imports,
    unused_results,
    unused_variables
)]
use crate::console::console_utils;
use fisco_bcos_rust_gears_sdk::bcos2sdk::bcos2client;
use fisco_bcos_rust_gears_sdk::bcos2sdk::bcos2client::Bcos2Client;
use fisco_bcos_rust_gears_sdk::bcos2sdk::bcossdkquery;
use fisco_bcos_rust_gears_sdk::bcos2sdk::bcossdkquery::json_hextoint;
use fisco_bcos_rust_gears_sdk::bcossdkutil::contractabi::ContractABI;
use fisco_bcos_rust_gears_sdk::bcossdkutil::contracthistory::ContractHistory;
use fisco_bcos_rust_gears_sdk::bcossdkutil::kisserror::KissError;
use fisco_bcos_rust_gears_sdk::bcossdkutil::solcompile::sol_compile;
use serde_json::{json, Value as JsonValue};
use std::time::Duration;

pub fn demo_deploy_simpleinfo(bcossdk: &mut Bcos2Client) -> Result<String, KissError> {
    let params: [String; 0] = [];
    let contract_name = "SimpleInfo";
    let compileres = sol_compile(
        contract_name,
        &bcossdk.config.configfile.as_ref().unwrap().as_str(),
    );
    println!("compile result:{:?}", compileres);

    let binfile = format!(
        "{}/{}.bin",
        bcossdk.config.common.contractpath,
        contract_name.to_string()
    );
    let v = bcossdk.deploy_file(binfile.as_str(), "");
    println!("request response {:?}", v);
    let response = v.unwrap();
    let txhash = response["result"].as_str().unwrap();
    let recepitresult = bcossdk.try_getTransactionReceipt(txhash, 3, false);
    //println!("receipt {:?}",recepitresult);

    let receipt = recepitresult.unwrap();
    console_utils::display_transaction_receipt(&receipt, &Option::from(None), &bcossdk.config);
    let addr: String = receipt["result"]["contractAddress"]
        .as_str()
        .unwrap()
        .to_string();
    let blocknum = json_hextoint(&receipt["result"]["blockNumber"]).unwrap();
    println!("deploy contract on block {}", blocknum);
    let chf = ContractHistory::history_file(bcossdk.config.common.contractpath.as_str());
    let res = ContractHistory::save_to_file(
        chf.as_str(),
        "bcos2",
        "SimpleInfo",
        addr.as_str(),
        blocknum as u64,
    );
    Ok(addr)
}

pub fn demo_simpleinfo_set(
    bcossdk: &mut Bcos2Client,
    address: &str,
    contract: &ContractABI,
) -> Result<JsonValue, KissError> {
    println!("\n>>>>>>>>>>>>>>>>>>>>demo simpleinfo set");

    let params: [String; 3] = [
        String::from("12347890abcefghe"),
        String::from("100"),
        String::from("40034be5fd46006238c04c2cedfe92dbddbdb651"),
    ];
    let res = bcossdk.sendRawTransactionGetReceipt(contract, address, "set", &params)?;
    console_utils::display_transaction_receipt(&res, &Option::from(contract), &bcossdk.config);
    let txhash = res["result"]["transactionHash"].as_str().unwrap();
    println!("\n>>>>>>>>>>>>>>>>>>>demo simpleinfo getTransactionByHash");
    let txdata = bcossdk.getTransactionByHash(txhash).unwrap();
    let blocknum = json_hextoint(&txdata["result"]["blockNumber"]);
    println!("tx {:?} on block {:?}", txhash, blocknum);
    let txinput = txdata["result"]["input"].as_str().unwrap();
    //println!("txinput str : {:?}",&txinput);
    let inputdecode = contract.decode_input_for_tx(txinput);
    println!("tx input :{:?}", inputdecode);
    Ok(txdata)
}

//---------------------------------------------------------
pub fn demo(configfile: &str) {
    let mut bcossdk = Bcos2Client::new_from_config(configfile).unwrap();
    let contract = ContractABI::new_by_name(
        "SimpleInfo",
        bcossdk.config.common.contractpath.as_str(),
        &bcossdk.hashtype,
    )
    .unwrap();
    let block_limit = bcossdk.getBlockLimit();
    println!("block limit {:?}", block_limit);

    println!("\n>>>>>>>>>>>>>>>>>>demo deploy contract");
    let newaddr = demo_deploy_simpleinfo(&mut bcossdk).unwrap();
    println!("new addr {}", &newaddr);

    let to_address = newaddr;

    let res = demo_simpleinfo_set(&mut bcossdk, to_address.as_str(), &contract);

    println!(">>>>>>>>>>>>>>>> call after transaction {:?}", res);
    let callvalue = bcossdk
        .call(&contract, &to_address, "getall", &[String::new()])
        .unwrap();
    let output = callvalue["result"]["output"].as_str().unwrap();

    let decodereuslt = contract.decode_output_byname("getall", output);
    println!("get function output: {:?}", decodereuslt);
    bcossdk.finish();
}
