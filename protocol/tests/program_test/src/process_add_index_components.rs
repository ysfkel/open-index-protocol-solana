use openindex_sdk::openindex::transaction::{
    add_index_components_transaction, create_mint_acccount_transaction,
};
use solana_sdk::signature::Signer;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use crate::{BanksClientResult, ProcessAddIndexComponentsResult, Setup};

pub async fn process_add_index_components(
    index_id: u64,
    controller_id: u64,
    manager: Pubkey,
    mints_count: u64,
    units: Vec<u64>,
    _setup: &Setup,
) -> ProcessAddIndexComponentsResult {
    let program_id = _setup.program_id;

    let mut mints = vec![];
    // Create mints
    for i in 1..=mints_count {
        let mint = Keypair::new();
        let create_mint_tx = create_mint_acccount_transaction(
            &_setup.payer,
            &mint,
            _setup.recent_blockhashes,
            &_setup.rent,
        );
        let _ = _setup
            .banks_client
            .process_transaction(create_mint_tx)
            .await;
        mints.push(mint.pubkey());
    }

    let transaction = add_index_components_transaction(
        &_setup.payer,
        _setup.program_id,
        index_id,
        controller_id,
        _setup.recent_blockhashes.clone(),
        mints.clone(),
        units.clone(),
    );

    let result = _setup.banks_client.process_transaction(transaction).await;

    ProcessAddIndexComponentsResult {
        index_id,
        controller_id,
        mints,
        units,
        result,
    }
}
