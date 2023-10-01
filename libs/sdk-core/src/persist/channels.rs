use crate::models::*;

use super::db::SqliteStorage;
use anyhow::Result;
use std::str::FromStr;

impl SqliteStorage {
    pub(crate) fn update_channels(&self, channels: &[Channel]) -> Result<()> {
        // insert all channels
        for c in channels.iter().cloned() {
            self.insert_or_update_channel(c)?
        }

        let funding_txs: Vec<String> = channels
            .iter()
            .cloned()
            .map(|c| format!("'{}'", c.funding_txid))
            .collect();

        // close channels not in list
        self.get_connection()?.execute(
            format!(
                "
                 UPDATE channels 
                 SET 
                  state=?1, 
                  closed_at = case when closed_at is null then unixepoch() else closed_at end 
                 where funding_txid not in ({})
                ",
                funding_txs.join(",")
            )
            .as_str(),
            (ChannelState::Closed.to_string(),),
        )?;

        Ok(())
    }

    pub(crate) fn list_channels(&self) -> Result<Vec<Channel>> {
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
                alias_remote
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
                })
            })?
            .map(|i| i.unwrap())
            .collect();

        Ok(channels)
    }

    fn insert_or_update_channel(&self, c: Channel) -> Result<()> {
        self.get_connection()?.execute(
            "INSERT INTO channels (
                   funding_txid, 
                   short_channel_id,
                   state,
                   spendable_msat, 
                   receivable_msat,
                   closed_at,
                   funding_outnum,
                   closed_at,
                   alias_local,
                   alias_remote
                  )
                  VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
                  ON CONFLICT(funding_txid) DO UPDATE SET
                   short_channel_id=excluded.short_channel_id,
                   state=excluded.state,
                   spendable_msat=excluded.spendable_msat,
                   receivable_msat=excluded.receivable_msat,
                   funding_outnum=excluded.funding_outnum,
                   closed_at=excluded.closed_at,
                   alias_local=excluded.alias_local,
                   alias_remote=excluded.alias_remote
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
                c.closed_at,
                c.alias_local,
                c.alias_remote,
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
        },
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
        },
    ];

    storage.update_channels(&channels).unwrap();
    let queried_channels = storage.list_channels().unwrap();
    assert_eq!(2, queried_channels.len());
    assert_eq!(channels[0], queried_channels[0]);
    assert!(queried_channels[1].closed_at.is_some());

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
        },
    ];
    assert_eq!(expected.len(), queried_channels.len());
    assert!(queried_channels[0].closed_at.is_some());
    assert!(queried_channels[1].closed_at.is_some());

    // test dedup channels in db
    storage.update_channels(&channels).unwrap();
    let queried_channels = storage.list_channels().unwrap();
    assert_eq!(channels.len(), queried_channels.len());
}
