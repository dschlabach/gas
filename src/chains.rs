pub const CHAINS: &[(u32, &str)] = &[
    (1, "Ethereum"),
    (8453, "Base"),
    (42161, "Arbitrum"),
    (137, "Polygon"),
    (10, "Optimism"),
];

pub fn get_chain_name(chain: u32) -> String {
    return CHAINS
        .iter()
        .find(|&&(chain_id, _)| chain_id == chain)
        .map_or(format!("UNKNOWN_CHAIN {chain}"), |(_, name)| {
            name.to_string()
        });
}
