use clap::Parser;
use dotenv;
use ethers::{prelude::*, utils::format_units};

mod chains;
use crate::chains::get_chain_name;

#[derive(Parser)]
#[command(
    author = "Daniel Schlabach",
    about = "A CLI for getting gas prices across different EVM chains."
)]
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Could not find .env file");

    let rpc_urls: Vec<String> = std::env::vars()
        .filter(|(name, _)| name.ends_with("RPC_URL"))
        .map(|(_, value)| value)
        .collect();

    let providers: Vec<Provider<Http>> = rpc_urls
        .iter()
        .map(|rpc_url| Provider::<Http>::try_from(rpc_url).expect("Error instantiating providers"))
        .collect();

    for provider in providers {
        let chain_id = provider.get_chainid().await?;
        let gas_price = provider.get_gas_price().await?;
        let block_number = provider.get_block_number().await?;

        let formatted_gas_price = format_units(gas_price, "gwei")?;
        let chain_name = get_chain_name(chain_id.as_u32());

        println!("{chain_name} \n    Gas: {formatted_gas_price} gwei\n    Block: {block_number}")
    }

    return Ok(());
}
