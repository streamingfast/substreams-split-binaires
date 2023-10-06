use common::pb::contract::v1 as contract;
use substreams::store::{StoreAdd, StoreAddInt64, StoreNew};

#[substreams::handlers::store]
fn store_stats(events: contract::Events, store: StoreAddInt64) {
    store.add(
        0,
        "changed_fees_collector_cut_per_millions:count",
        events.changed_fees_collector_cut_per_millions.len() as i64,
    );
    store.add(
        0,
        "changed_publication_fees:count",
        events.changed_publication_fees.len() as i64,
    );
    store.add(
        0,
        "changed_royalties_cut_per_millions:count",
        events.changed_royalties_cut_per_millions.len() as i64,
    );
    store.add(
        0,
        "fees_collector_sets:count",
        events.fees_collector_sets.len() as i64,
    );
    store.add(
        0,
        "meta_transaction_executeds:count",
        events.meta_transaction_executeds.len() as i64,
    );
    store.add(
        0,
        "order_cancelleds:count",
        events.order_cancelleds.len() as i64,
    );
    store.add(
        0,
        "order_createds:count",
        events.order_createds.len() as i64,
    );
    store.add(
        0,
        "order_successfuls:count",
        events.order_successfuls.len() as i64,
    );
    store.add(
        0,
        "ownership_transferreds:count",
        events.ownership_transferreds.len() as i64,
    );
    store.add(0, "pauseds:count", events.pauseds.len() as i64);
    store.add(
        0,
        "royalties_manager_sets:count",
        events.royalties_manager_sets.len() as i64,
    );
    store.add(0, "unpauseds:count", events.unpauseds.len() as i64);
}
