use common::abi;
use common::pb::contract::v1 as contract;
use hex_literal::hex;
use substreams::store::{DeltaInt64, Deltas};
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

const TRACKED_CONTRACT: [u8; 20] = hex!("480a0f4e360e8964e68858dd231c2922f1df45ef");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        changed_fees_collector_cut_per_millions: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::ChangedFeesCollectorCutPerMillion::match_and_decode(log) {
                            return Some(contract::ChangedFeesCollectorCutPerMillion {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                fees_collector_cut_per_million: event.fees_collector_cut_per_million.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        changed_publication_fees: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::ChangedPublicationFee::match_and_decode(log) {
                            return Some(contract::ChangedPublicationFee {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                publication_fee: event.publication_fee.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        changed_royalties_cut_per_millions: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::ChangedRoyaltiesCutPerMillion::match_and_decode(log) {
                            return Some(contract::ChangedRoyaltiesCutPerMillion {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                royalties_cut_per_million: event.royalties_cut_per_million.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        fees_collector_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::FeesCollectorSet::match_and_decode(log) {
                            return Some(contract::FeesCollectorSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_fees_collector: event.new_fees_collector,
                                old_fees_collector: event.old_fees_collector,
                            });
                        }

                        None
                })
            })
            .collect(),
        meta_transaction_executeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::MetaTransactionExecuted::match_and_decode(log) {
                            return Some(contract::MetaTransactionExecuted {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                function_signature: event.function_signature,
                                relayer_address: event.relayer_address,
                                user_address: event.user_address,
                            });
                        }

                        None
                })
            })
            .collect(),
        order_cancelleds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::OrderCancelled::match_and_decode(log) {
                            return Some(contract::OrderCancelled {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                asset_id: event.asset_id.to_string(),
                                id: Vec::from(event.id),
                                nft_address: event.nft_address,
                                seller: event.seller,
                            });
                        }

                        None
                })
            })
            .collect(),
        order_createds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::OrderCreated::match_and_decode(log) {
                            return Some(contract::OrderCreated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                asset_id: event.asset_id.to_string(),
                                expires_at: event.expires_at.to_string(),
                                id: Vec::from(event.id),
                                nft_address: event.nft_address,
                                price_in_wei: event.price_in_wei.to_string(),
                                seller: event.seller,
                            });
                        }

                        None
                })
            })
            .collect(),
        order_successfuls: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::OrderSuccessful::match_and_decode(log) {
                            return Some(contract::OrderSuccessful {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                asset_id: event.asset_id.to_string(),
                                buyer: event.buyer,
                                id: Vec::from(event.id),
                                nft_address: event.nft_address,
                                seller: event.seller,
                                total_price: event.total_price.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        ownership_transferreds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::OwnershipTransferred::match_and_decode(log) {
                            return Some(contract::OwnershipTransferred {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_owner: event.new_owner,
                                previous_owner: event.previous_owner,
                            });
                        }

                        None
                })
            })
            .collect(),
        pauseds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Paused::match_and_decode(log) {
                            return Some(contract::Paused {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                account: event.account,
                            });
                        }

                        None
                })
            })
            .collect(),
        royalties_manager_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::RoyaltiesManagerSet::match_and_decode(log) {
                            return Some(contract::RoyaltiesManagerSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_royalties_manager: event.new_royalties_manager,
                                old_royalties_manager: event.old_royalties_manager,
                            });
                        }

                        None
                })
            })
            .collect(),
        unpauseds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Unpaused::match_and_decode(log) {
                            return Some(contract::Unpaused {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                account: event.account,
                            });
                        }

                        None
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(
    events: contract::Events,
    store: Deltas<DeltaInt64>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    for delta in store.into_iter() {
        substreams::log::info!("Key {} => {}", delta.key, delta.new_value);
    }

    // Initialize Database Changes container
    let mut tables = substreams_database_change::tables::Tables::new();

    // Loop over all the abis events to create database changes
    events
        .changed_fees_collector_cut_per_millions
        .into_iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "changed_fees_collector_cut_per_millions",
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
                .set("evt_block_time", evt.evt_block_time.unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set(
                    "fees_collector_cut_per_million",
                    evt.fees_collector_cut_per_million.to_string(),
                );
        });
    events.changed_publication_fees.into_iter().for_each(|evt| {
        tables
            .create_row(
                "changed_publication_fees",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("publication_fee", evt.publication_fee.to_string());
    });
    events
        .changed_royalties_cut_per_millions
        .into_iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "changed_royalties_cut_per_millions",
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
                .set("evt_block_time", evt.evt_block_time.unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set(
                    "royalties_cut_per_million",
                    evt.royalties_cut_per_million.to_string(),
                );
        });
    events.fees_collector_sets.into_iter().for_each(|evt| {
        tables
            .create_row(
                "fees_collector_sets",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "new_fees_collector",
                Hex(&evt.new_fees_collector).to_string(),
            )
            .set(
                "old_fees_collector",
                Hex(&evt.old_fees_collector).to_string(),
            );
    });
    events
        .meta_transaction_executeds
        .into_iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "meta_transaction_executeds",
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
                .set("evt_block_time", evt.evt_block_time.unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set(
                    "function_signature",
                    Hex(&evt.function_signature).to_string(),
                )
                .set("relayer_address", Hex(&evt.relayer_address).to_string())
                .set("user_address", Hex(&evt.user_address).to_string());
        });
    events.order_cancelleds.into_iter().for_each(|evt| {
        tables
            .create_row(
                "order_cancelleds",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("asset_id", evt.asset_id.to_string())
            .set("id", Hex(&evt.id).to_string())
            .set("nft_address", Hex(&evt.nft_address).to_string())
            .set("seller", Hex(&evt.seller).to_string());
    });
    events.order_createds.into_iter().for_each(|evt| {
        tables
            .create_row(
                "order_createds",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("asset_id", evt.asset_id.to_string())
            .set("expires_at", evt.expires_at.to_string())
            .set("id", Hex(&evt.id).to_string())
            .set("nft_address", Hex(&evt.nft_address).to_string())
            .set("price_in_wei", evt.price_in_wei.to_string())
            .set("seller", Hex(&evt.seller).to_string());
    });
    events.order_successfuls.into_iter().for_each(|evt| {
        tables
            .create_row(
                "order_successfuls",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("asset_id", evt.asset_id.to_string())
            .set("buyer", Hex(&evt.buyer).to_string())
            .set("id", Hex(&evt.id).to_string())
            .set("nft_address", Hex(&evt.nft_address).to_string())
            .set("seller", Hex(&evt.seller).to_string())
            .set("total_price", evt.total_price.to_string());
    });
    events.ownership_transferreds.into_iter().for_each(|evt| {
        tables
            .create_row(
                "ownership_transferreds",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.pauseds.into_iter().for_each(|evt| {
        tables
            .create_row("pauseds", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.royalties_manager_sets.into_iter().for_each(|evt| {
        tables
            .create_row(
                "royalties_manager_sets",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "new_royalties_manager",
                Hex(&evt.new_royalties_manager).to_string(),
            )
            .set(
                "old_royalties_manager",
                Hex(&evt.old_royalties_manager).to_string(),
            );
    });
    events.unpauseds.into_iter().for_each(|evt| {
        tables
            .create_row(
                "unpauseds",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });

    Ok(tables.to_database_changes())
}
