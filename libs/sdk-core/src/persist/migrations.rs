pub(crate) fn current_migrations() -> Vec<&'static str> {
    vec![
        "
        CREATE TABLE IF NOT EXISTS payments (
          payment_type TEXT NOT NULL check( payment_type in('sent', 'received')),
          payment_hash TEXT NOT NULL PRIMARY KEY,
          payment_time INTEGER NOT NULL,
          label TEXT,
          destination_pubkey TEXT NOT NULL,
          amount_msats INTEGER NOT NULL,
          fee_msat INTEGER NOT NULL,
          payment_preimage TEXT,
          keysend INTEGER NOT NULL,                  
          bolt11 TEXT,
          pending INTEGER NOT NULL,
          description TEXT
        ) STRICT;  
        
        CREATE TABLE IF NOT EXISTS settings (
         key TEXT NOT NULL PRIMARY KEY,
         value TEXT NOT NULL
        ) STRICT;

        CREATE TABLE IF NOT EXISTS cached_items (
         key TEXT NOT NULL PRIMARY KEY,
         value TEXT NOT NULL
        ) STRICT;
        
        CREATE TABLE IF NOT EXISTS swaps (
          bitcoin_address TEXT PRIMARY KEY NOT NULL,
          created_at INTEGER DEFAULT CURRENT_TIMESTAMP,
          lock_height INTEGER NOT NULL,
          payment_hash BLOB NOT NULL UNIQUE,
          preimage BLOB NOT NULL UNIQUE,
          private_key BLOB NOT NULL UNIQUE,
          public_key BLOB NOT NULL UNIQUE,
          swapper_public_key BLOB NOT NULL UNIQUE,
          script BLOB NOT NULL UNIQUE,
          bolt11 TEXT,
          paid_sats INTEGER NOT NULL DEFAULT 0,
          confirmed_sats INTEGER NOT NULL DEFAULT 0,               
          status INTEGER NOT NULL DEFAULT 0,
          refund_tx_ids TEXT NOT NULL, 
          confirmed_tx_ids TEXT NOT NULL
        ) STRICT;
       ",
        "
       CREATE TABLE channels (
        funding_txid TEXT NOT NULL PRIMARY KEY,
        short_channel_id TEXT,
        state TEXT NOT NULL check( state in('PendingOpen', 'Opened', 'PendingClose', 'Closed')),
        spendable_msat INTEGER NOT NULL,
        receivable_msat INTEGER NOT NULL,
        closed_at INTEGER
       ) STRICT;
       ",
       "
       ALTER TABLE payments RENAME TO old_payments;

       CREATE TABLE IF NOT EXISTS payments (
        id TEXT NOT NULL PRIMARY KEY,
        payment_type TEXT NOT NULL check( payment_type in('Sent', 'Received', 'ClosedChannel')),             
        payment_time INTEGER NOT NULL,             
        amount_msat INTEGER NOT NULL,
        fee_msat INTEGER NOT NULL,             
        pending INTEGER NOT NULL,
        description TEXT,
        details TEXT
       ) STRICT;
       
       INSERT INTO payments
        (id, payment_type, payment_time, amount_msat, fee_msat, pending, description, details)
        SELECT 
         payment_hash, 
         case when payment_type = 'received' then 'Received' else 'Sent' end, 
         payment_time, 
         amount_msats,
         fee_msat, 
         pending, 
         description, 
         json_object(
          'payment_hash', payment_hash, 
          'label', label, 
          'destination_pubkey', destination_pubkey, 
          'payment_preimage', payment_preimage, 
          'keysend', CASE keysend WHEN 1 THEN json('true') ELSE json('false') END, 
          'bolt11', bolt11
         )
        FROM old_payments;
       
       DROP TABLE old_payments;            
       ",

       "
       ALTER TABLE swaps ADD COLUMN min_allowed_deposit INTEGER NOT NULL;
       ALTER TABLE swaps ADD COLUMN max_allowed_deposit INTEGER NOT NULL;
       ",
       "UPDATE payments SET fee_msat = ABS(fee_msat) WHERE fee_msat < 0",

       "
       ALTER TABLE swaps RENAME TO old_swaps;

       CREATE TABLE IF NOT EXISTS swaps (
        bitcoin_address TEXT PRIMARY KEY NOT NULL,
        created_at INTEGER DEFAULT CURRENT_TIMESTAMP,
        lock_height INTEGER NOT NULL,
        payment_hash BLOB NOT NULL UNIQUE,
        preimage BLOB NOT NULL UNIQUE,
        private_key BLOB NOT NULL UNIQUE,
        public_key BLOB NOT NULL UNIQUE,
        swapper_public_key BLOB NOT NULL UNIQUE,
        script BLOB NOT NULL UNIQUE,
        bolt11 TEXT,
        paid_sats INTEGER NOT NULL DEFAULT 0,
        unconfirmed_sats INTEGER NOT NULL DEFAULT 0, 
        confirmed_sats INTEGER NOT NULL DEFAULT 0,               
        status INTEGER NOT NULL DEFAULT 0,
        refund_tx_ids TEXT NOT NULL,  
        unconfirmed_tx_ids TEXT NOT NULL,
        confirmed_tx_ids TEXT NOT NULL,
        min_allowed_deposit INTEGER NOT NULL,
        max_allowed_deposit INTEGER NOT NULL,
        last_redeem_error TEXT   
       ) STRICT;
       
       INSERT INTO swaps
        (
         bitcoin_address, 
         created_at,
         lock_height,
         payment_hash,
         preimage,
         private_key,
         public_key,
         swapper_public_key,
         script,
         bolt11,
         paid_sats,
         unconfirmed_sats,
         confirmed_sats,
         status,
         refund_tx_ids,
         unconfirmed_tx_ids,
         confirmed_tx_ids,
         min_allowed_deposit,
         max_allowed_deposit,
         last_redeem_error
        )
        SELECT 
         bitcoin_address, 
         created_at,
         lock_height,
         payment_hash,
         preimage,
         private_key,
         public_key,
         swapper_public_key,
         script,
         bolt11,
         paid_sats,
         0,
         confirmed_sats,
         status,
         refund_tx_ids,
         '[]',
         confirmed_tx_ids,
         min_allowed_deposit,
         max_allowed_deposit,
         NULL
        FROM old_swaps;
       
       DROP TABLE old_swaps;            
       ",
       "
       CREATE TABLE IF NOT EXISTS payments_external_info (
        payment_id TEXT NOT NULL PRIMARY KEY,
        lnurl_success_action TEXT,
        FOREIGN KEY(payment_id) REFERENCES payments(id)
       ) STRICT;
       ",
       "ALTER TABLE payments_external_info ADD COLUMN ln_address TEXT;",
       "ALTER TABLE payments_external_info ADD COLUMN lnurl_metadata TEXT;",
       "
       ALTER TABLE swaps RENAME TO old_swaps;

       CREATE TABLE IF NOT EXISTS swaps (
        bitcoin_address TEXT PRIMARY KEY NOT NULL,
        created_at INTEGER DEFAULT CURRENT_TIMESTAMP,
        lock_height INTEGER NOT NULL,
        payment_hash BLOB NOT NULL UNIQUE,
        preimage BLOB NOT NULL UNIQUE,
        private_key BLOB NOT NULL UNIQUE,
        public_key BLOB NOT NULL UNIQUE,
        swapper_public_key BLOB NOT NULL UNIQUE,
        script BLOB NOT NULL UNIQUE,
        min_allowed_deposit INTEGER NOT NULL,
        max_allowed_deposit INTEGER NOT NULL
       ) STRICT;

       CREATE TABLE IF NOT EXISTS swaps_info (
        bitcoin_address TEXT PRIMARY KEY NOT NULL,
        bolt11 TEXT,
        paid_sats INTEGER NOT NULL DEFAULT 0,
        unconfirmed_sats INTEGER NOT NULL DEFAULT 0, 
        confirmed_sats INTEGER NOT NULL DEFAULT 0,               
        status INTEGER NOT NULL DEFAULT 0,        
        unconfirmed_tx_ids TEXT NOT NULL,
        confirmed_tx_ids TEXT NOT NULL,        
        last_redeem_error TEXT,
        FOREIGN KEY(bitcoin_address) REFERENCES swaps(bitcoin_address)
       ) STRICT;

       CREATE TABLE IF NOT EXISTS swap_refunds (
        bitcoin_address TEXT NOT NULL,        
        refund_tx_id TEXT NOT NULL,
        PRIMARY KEY (bitcoin_address, refund_tx_id),
        FOREIGN KEY(bitcoin_address) REFERENCES swaps(bitcoin_address)    
       ) STRICT;
       
       INSERT INTO swaps
        (
         bitcoin_address, 
         created_at,
         lock_height,
         payment_hash,
         preimage,
         private_key,
         public_key,
         swapper_public_key,
         script,
         min_allowed_deposit,
         max_allowed_deposit
        )
        SELECT 
         bitcoin_address, 
         created_at,
         lock_height,
         payment_hash,
         preimage,
         private_key,
         public_key,
         swapper_public_key,
         script,
         min_allowed_deposit,
         max_allowed_deposit         
        FROM old_swaps;        

       INSERT INTO swaps_info
        (
         bitcoin_address,
         bolt11,
         paid_sats,
         unconfirmed_sats,
         confirmed_sats,
         status,         
         unconfirmed_tx_ids,
         confirmed_tx_ids,
         last_redeem_error
        )
        SELECT 
         bitcoin_address,
         bolt11,
         paid_sats,
         unconfirmed_sats,
         confirmed_sats,
         status,         
         unconfirmed_tx_ids,
         confirmed_tx_ids,
         last_redeem_error
        FROM old_swaps;

       DROP TABLE old_swaps;            
       "
    ]
}
