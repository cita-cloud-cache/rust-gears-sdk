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
use std::time::Duration;

use crate::bcossdkutil::contracthistory::ContractHistory;
use fisco_bcos_rust_gears_sdk::bcos2sdk::bcos2client::Bcos2Client;
use fisco_bcos_rust_gears_sdk::bcos2sdk::bcossdkquery::json_hextoint;
use fisco_bcos_rust_gears_sdk::bcossdkutil::contractabi::ContractABI;
use fisco_bcos_rust_gears_sdk::bcossdkutil::fileutils;
use fisco_bcos_rust_gears_sdk::bcossdkutil::kisserror::KissError;
use fisco_bcos_rust_gears_sdk::bcossdkutil::solcompile::sol_compile;
use serde_json::{json, Value as JsonValue};
use std::thread;

pub fn demo_deploy(bcossdk: &mut Bcos2Client, contract: &ContractABI) -> Result<String, KissError> {
    let contract_name = "NeedInit";
    let compileres = sol_compile(
        contract_name,
        &bcossdk.config.configfile.as_ref().unwrap().as_str(),
    );
    println!("compile result:{:?}", compileres);

    let params: [String; 2] = ["default text 009".to_string(), "199".to_string()];

    let binfile = format!(
        "{}/{}.bin",
        bcossdk.config.common.contractpath,
        contract_name.to_string()
    );
    let paramcode = contract
        .encode_construtor_input(Vec::new(), &params, true)
        .unwrap();
    let v = bcossdk.deploy_file(binfile.as_str(), paramcode.as_str());
    println!("request response {:?}", v);
    let response = v.unwrap();
    let txhash = response["result"].as_str().unwrap();
    let recepitresult = bcossdk.try_getTransactionReceipt(txhash, 3, false);
    println!("receipt {:?}", recepitresult);
    let receipt = recepitresult.unwrap();
    let addr: String = receipt["result"]["contractAddress"]
        .as_str()
        .unwrap()
        .to_string();
    let blocknum = json_hextoint(&receipt["result"]["blockNumber"]).unwrap();
    println!("deploy contract on block {}", blocknum);
    let history_file = ContractHistory::history_file(bcossdk.config.common.contractpath.as_str());
    let res = ContractHistory::save_to_file(
        history_file.as_str(),
        "bcos2",
        "NeedInit",
        addr.as_str(),
        blocknum as u64,
    );

    Ok(addr)
}

//---------------------------------------------------------
pub fn demo(configfile: &str) {
    let contract_name = "NeedInit";
    let mut bcossdk = Bcos2Client::new_from_config(configfile).unwrap();

    let contract = ContractABI::new_by_name(
        contract_name,
        bcossdk.config.common.contractpath.as_str(),
        &bcossdk.hashtype,
    )
    .unwrap();
    let block_limit = bcossdk.getBlockLimit();
    println!("block limit {:?}", block_limit);

    println!("\n>>>>>>>>>>>>>>>>>>demo deploy contract");
    let newaddr = demo_deploy(&mut bcossdk, &contract).unwrap();
    println!("new addr {}", &newaddr);

    let to_address = newaddr;

    println!(">>>>>>>>>>>>>>>> call after transaction");
    let callvalue = bcossdk
        .call(&contract, &to_address, "get", &[String::new()])
        .unwrap();
    let output = callvalue["result"]["output"].as_str().unwrap();

    let decodereuslt = contract.decode_output_byname("get", output);
    println!("get function output: {:?}", decodereuslt);

    let history_file = ContractHistory::history_file(bcossdk.config.common.contractpath.as_str());
    let lastest =
        ContractHistory::get_last_from_file(history_file.as_str(), "bcos2", contract_name);
    println!("demo contract {} done", lastest.unwrap());
    println!("demo on : {:?}", bcossdk.getNodeVersion());
    bcossdk.finish();
}
