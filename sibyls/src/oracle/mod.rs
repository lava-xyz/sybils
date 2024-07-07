use crate::{AssetPair, AssetPairInfo, OracleConfig};
use secp256k1_zkp::KeyPair;

mod error;
pub use error::OracleError;
pub use error::Result;

#[derive(Clone)]
pub struct EventData {
    pub maturation: OffsetDateTime,
    pub asset_pair: AssetPair,
    pub outstanding_sk_nonces: Vec<[u8; 32]>,
}

#[derive(Clone)]
pub struct Oracle {
    pub oracle_config: OracleConfig,
    asset_pair_info: AssetPairInfo,
    pub event_database: EventStorage,
    keypair: KeyPair,
}

impl Oracle {
    pub fn new(
        oracle_config: OracleConfig,
        asset_pair_info: AssetPairInfo,
        keypair: KeyPair,
        database_url: &String,
    ) -> Result<Oracle> {
        if !oracle_config.announcement_offset.is_positive() {
            return Err(OracleError::InvalidAnnouncementTimeError(
                oracle_config.announcement_offset,
            ));
        }

        let event_database = EventStorage::new(database_url, asset_pair_info.asset_pair)?;

        Ok(Oracle {
            oracle_config,
            asset_pair_info,
            event_database,
            keypair,
        })
    }
}

use crate::db::EventStorage;
pub use dlc_messages::oracle_msgs::EventDescriptor;
use time::OffsetDateTime;

pub mod oracle_scheduler;
pub mod pricefeeds;
