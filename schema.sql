
CREATE TABLE IF NOT EXISTS changed_fees_collector_cut_per_millions (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "fees_collector_cut_per_million" DECIMAL
);
CREATE TABLE IF NOT EXISTS changed_publication_fees (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "publication_fee" DECIMAL
);
CREATE TABLE IF NOT EXISTS changed_royalties_cut_per_millions (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "royalties_cut_per_million" DECIMAL
);
CREATE TABLE IF NOT EXISTS fees_collector_sets (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_fees_collector" VARCHAR(40),
    "old_fees_collector" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS meta_transaction_executeds (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "function_signature" TEXT,
    "relayer_address" VARCHAR(40),
    "user_address" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS order_cancelleds (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "asset_id" DECIMAL,
    "id" TEXT,
    "nft_address" VARCHAR(40),
    "seller" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS order_createds (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "asset_id" DECIMAL,
    "expires_at" DECIMAL,
    "id" TEXT,
    "nft_address" VARCHAR(40),
    "price_in_wei" DECIMAL,
    "seller" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS order_successfuls (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "asset_id" DECIMAL,
    "buyer" VARCHAR(40),
    "id" TEXT,
    "nft_address" VARCHAR(40),
    "seller" VARCHAR(40),
    "total_price" DECIMAL
);
CREATE TABLE IF NOT EXISTS ownership_transferreds (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS pauseds (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "account" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS royalties_manager_sets (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_royalties_manager" VARCHAR(40),
    "old_royalties_manager" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS unpauseds (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "account" VARCHAR(40)
);