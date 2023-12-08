use std::collections::HashMap;

use jsonrpc::simple_http::SimpleHttpTransport;
use jsonrpc::Client;

use serde_json::value::RawValue;
use zcash_client_backend::decrypt_transaction;
use zcash_client_backend::keys::UnifiedFullViewingKey;
use zcash_primitives::consensus::{BlockHeight, BranchId};
use zcash_primitives::transaction::Transaction;
use zcash_primitives::zip32::AccountId;
use zebra_scan::scan::sapling_key_to_scan_block_keys;
use zebra_scan::{storage::Storage, Config};

#[cfg(test)]
mod tests;

pub fn main() {
    let network = zcash_primitives::consensus::Network::MainNetwork;
    let storage = Storage::new(&Config::default(), zebra_network(&network), true);
    let keys = storage.sapling_keys();

    for (key, _) in keys.iter() {
        let dfvk = sapling_key_to_scan_block_keys(key, zebra_network(&network)).unwrap();
        let dfvk = dfvk.0.into_iter().next().unwrap();
        let ufvk = UnifiedFullViewingKey::new(Some(dfvk), None).unwrap();
        let ufvk_with_acc_id = HashMap::from([(AccountId::from(1), ufvk)]);
        for (height, txids) in storage.sapling_results(key) {
            let height = BlockHeight::from_u32(height.0);
            let branch_id = BranchId::for_height(&network, height);

            for txid in txids.iter() {
                let txid = format!("\"{}\"", hex::encode(<[u8; 32]>::from(*txid)));
                let tx_bytes = hex::decode(&get_tx_via_rpc(txid)).unwrap();
                let tx = Transaction::read(&tx_bytes[..], branch_id).unwrap();
                let outputs = decrypt_transaction(&network, height, &tx, &ufvk_with_acc_id);

                for output in outputs.iter() {
                    let memo = memo_bytes_to_string(output.memo.as_array());

                    if !memo.is_empty() && !memo.contains("LIKE:") && !memo.contains("VOTE:") {
                        println!("{memo}\n");
                    }
                }
            }
        }
    }
}

/// Trims trailing zeroes from a memo, and returns the memo as a string.
fn memo_bytes_to_string(memo: &[u8; 512]) -> &str {
    match memo.iter().rposition(|&byte| byte != 0) {
        Some(index) => std::str::from_utf8(&memo[..=index]).unwrap_or(""),
        None => "",
    }
}

/// Uses the `getrawtransaction` RPC to retrieve a transaction by its TXID.
fn get_tx_via_rpc(txid: String) -> String {
    let transport = SimpleHttpTransport::builder()
        .url("127.0.0.1:8232")
        .unwrap()
        .build();
    let client = Client::with_transport(transport);
    let params = [RawValue::from_string(txid).unwrap()];
    let request = client.build_request("getrawtransaction", &params);
    let response = client.send_request(request).expect("send_request failed");

    response.result().unwrap()
}

/// Converts [`zcash_primitives::consensus::Network`] to [`zebra_chain::parameters::Network`].
fn zebra_network(
    network: &zcash_primitives::consensus::Network,
) -> zebra_chain::parameters::Network {
    match network {
        zcash_primitives::consensus::Network::MainNetwork => {
            zebra_chain::parameters::Network::Mainnet
        }
        zcash_primitives::consensus::Network::TestNetwork => {
            zebra_chain::parameters::Network::Testnet
        }
    }
}
