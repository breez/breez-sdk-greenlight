use crate::models::*;
use std::collections::HashMap;

use super::{db::SqliteStorage, error::PersistResult};
use std::str::FromStr;

impl SqliteStorage {
    /// Expects a full list of (non-closed) channels.
    ///
    /// Any known channel that is missing from the list, will be marked as closed. When doing so, the
    /// closing-related fields `closed_at` and `closing_txid` are not set, because doing so would require
    /// a chain service lookup. Instead, they will be set on first lookup in
    /// [BreezServices::closed_channel_to_transaction]
    pub(crate) fn update_channels(&self, fetched_channels: &[Channel]) -> PersistResult<()> {
        // create a hash map of the channels before the update
        let channels_before_update = self
            .list_channels()?
            .into_iter()
            .map(|c| (c.funding_txid.clone(), c))
            .collect::<HashMap<_, _>>();

        // merge the closed_at and closed_txid from the persisted channels into the fetched channels
        let new_channels: Vec<Channel> = fetched_channels
            .iter()
            .map(|c| {
                let persisted_channel = channels_before_update.get(&c.funding_txid);
                let mut cloned_channel = c.clone();
                if let Some(unwrapped_channel) = persisted_channel {
                    cloned_channel.closed_at = unwrapped_channel.closed_at;
                    cloned_channel.closing_txid = unwrapped_channel.closing_txid.clone();
                }
                cloned_channel
            })
            .collect();

        // insert all channels
        for c in new_channels.iter().cloned() {
            self.insert_or_update_channel(c)?
        }

        let funding_txs: Vec<String> = new_channels
            .iter()
            .cloned()
            .map(|c| format!("'{}'", c.funding_txid))
            .collect();

        // Close channels not in the list provided
        self.get_connection()?.execute(
            format!(
                "
                 UPDATE channels 
                 SET 
                  state=?1
                 where funding_txid not in ({})
                ",
                funding_txs.join(",")
            )
            .as_str(),
            (ChannelState::Closed.to_string(),),
        )?;

        Ok(())
    }

    pub(crate) fn list_channels(&self) -> PersistResult<Vec<Channel>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(
            "
               SELECT
                funding_txid, 
                short_channel_id,
                state, 
                spendable_msat, 
                receivable_msat,
                closed_at,
                funding_outnum,
                alias_local,
                alias_remote,
                closing_txid
               FROM channels             
             ",
        )?;
        let channels: Vec<Channel> = stmt
            .query_map([], |row| {
                let state_str: String = row.get(2)?;
                Ok(Channel {
                    funding_txid: row.get(0)?,
                    short_channel_id: row.get(1)?,
                    state: ChannelState::from_str(state_str.as_str())
                        .unwrap_or(ChannelState::Closed),
                    spendable_msat: row.get(3)?,
                    receivable_msat: row.get(4)?,
                    closed_at: row.get(5)?,
                    funding_outnum: row.get(6)?,
                    alias_local: row.get(7)?,
                    alias_remote: row.get(8)?,
                    closing_txid: row.get(9)?,
                    htlc: Vec::new(),
                })
            })?
            .map(|i| i.unwrap())
            .collect();

        Ok(channels)
    }

    pub(crate) fn insert_or_update_channel(&self, c: Channel) -> PersistResult<()> {
        self.get_connection()?.execute(
            "INSERT OR REPLACE INTO channels (
                   funding_txid, 
                   short_channel_id,
                   state,
                   spendable_msat, 
                   receivable_msat,
                   closed_at,
                   funding_outnum,                   
                   alias_local,
                   alias_remote,
                   closing_txid
                  )
                  VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
               ",
            (
                c.funding_txid,
                c.short_channel_id,
                c.state.to_string(),
                c.spendable_msat,
                c.receivable_msat,
                match c.state {
                    ChannelState::Opened | ChannelState::PendingOpen => None,
                    _ => c.closed_at,
                },
                c.funding_outnum,
                c.alias_local,
                c.alias_remote,
                c.closing_txid,
            ),
        )?;
        Ok(())
    }
}

#[test]
fn test_simple_sync_channels() {
    use crate::persist::test_utils;

    let storage = SqliteStorage::new(test_utils::create_test_sql_dir());

    storage.init().unwrap();
    let channels = vec![
        Channel {
            funding_txid: "123".to_string(),
            short_channel_id: "10x11x12".to_string(),
            state: ChannelState::Opened,
            spendable_msat: 100,
            receivable_msat: 1000,
            closed_at: None,
            funding_outnum: None,
            alias_local: None,
            alias_remote: None,
            closing_txid: None,
            htlc: Vec::new(),
        },
        Channel {
            funding_txid: "456".to_string(),
            short_channel_id: "13x14x15".to_string(),
            state: ChannelState::Opened,
            spendable_msat: 200,
            receivable_msat: 2000,
            closed_at: None,
            funding_outnum: None,
            alias_local: None,
            alias_remote: None,
            closing_txid: None,
            htlc: Vec::new(),
        },
    ];

    storage.update_channels(&channels).unwrap();
    let queried_channels = storage.list_channels().unwrap();
    assert_eq!(channels, queried_channels);

    storage.update_channels(&channels).unwrap();
    let queried_channels = storage.list_channels().unwrap();
    assert_eq!(channels, queried_channels);
}

#[test]
fn test_sync_closed_channels() {
    use crate::persist::test_utils;

    let storage = SqliteStorage::new(test_utils::create_test_sql_dir());

    storage.init().unwrap();
    let channels = vec![
        Channel {
            funding_txid: "123".to_string(),
            short_channel_id: "10x11x12".to_string(),
            state: ChannelState::Opened,
            spendable_msat: 100,
            receivable_msat: 1000,
            closed_at: None,
            funding_outnum: None,
            alias_local: None,
            alias_remote: None,
            closing_txid: None,
            htlc: Vec::new(),
        },
        // Simulate closed channel that was persisted with closed_at and closing_txid
        Channel {
            funding_txid: "456".to_string(),
            short_channel_id: "13x14x15".to_string(),
            state: ChannelState::Closed,
            spendable_msat: 200,
            receivable_msat: 2000,
            closed_at: Some(1),
            funding_outnum: None,
            alias_local: None,
            alias_remote: None,
            closing_txid: Some("a".into()),
            htlc: Vec::new(),
        },
    ];

    storage.update_channels(&channels).unwrap();
    let queried_channels = storage.list_channels().unwrap();
    assert_eq!(2, queried_channels.len());
    assert_eq!(channels[0], queried_channels[0]);
    assert!(queried_channels[1].closed_at.is_some());
    assert!(queried_channels[1].closing_txid.is_some());

    storage.update_channels(&channels).unwrap();
    let queried_channels = storage.list_channels().unwrap();
    assert_eq!(channels[0], queried_channels[0]);

    // test all channels were closed
    storage.update_channels(&Vec::new()).unwrap();
    let queried_channels = storage.list_channels().unwrap();
    let expected = vec![
        Channel {
            funding_txid: "123".to_string(),
            short_channel_id: "10x11x12".to_string(),
            state: ChannelState::Closed,
            spendable_msat: 100,
            receivable_msat: 1000,
            closed_at: None,
            funding_outnum: None,
            alias_local: None,
            alias_remote: None,
            closing_txid: None,
            htlc: Vec::new(),
        },
        Channel {
            funding_txid: "456".to_string(),
            short_channel_id: "13x14x15".to_string(),
            state: ChannelState::Closed,
            spendable_msat: 200,
            receivable_msat: 2000,
            closed_at: None,
            funding_outnum: None,
            alias_local: None,
            alias_remote: None,
            closing_txid: None,
            htlc: Vec::new(),
        },
    ];
    assert_eq!(expected.len(), queried_channels.len());
    // For channel inserted WITHOUT closed_at and closing_txid,
    // the closing-related fields are empty on channel queried directly from DB
    assert!(queried_channels[0].closed_at.is_none());
    assert!(queried_channels[0].closing_txid.is_none());
    // For channel inserted WITH closed_at and closing_txid (as for example after a chain service lookup),
    // the closing-related fields are not empty on channel queried directly from DB
    assert!(queried_channels[1].closed_at.is_some());
    assert!(queried_channels[1].closing_txid.is_some());

    // test dedup channels in db
    storage.update_channels(&channels).unwrap();
    let queried_channels = storage.list_channels().unwrap();
    assert_eq!(channels.len(), queried_channels.len());
}
