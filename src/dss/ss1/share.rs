use errors::*;
use share::IsShare;
use super::serialize::{share_from_string, share_to_string};

pub use dss::metadata::MetaData;

/// A share identified by an `id`, a threshold `k`, a number of total shares `n`,
/// the `data` held in the share, and the share's `metadata`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Share {
    /// The identifier of the share (varies between 1 and n where n is the total number of generated shares)
    pub id: u8,
    /// The number of shares necessary to recover the secret, aka a threshold
    pub threshold: u8,
    /// The total number of shares that have been dealt
    pub shares_count: u8,
    /// The share data itself
    pub data: Vec<u8>,
    /// The hash value common to the whole deal
    pub hash: Vec<u8>,
    /// The metadata associated with this share
    pub metadata: Option<MetaData>,
}

impl Share {
    /// Format this share a string suitable for sharing
    /// over an ASCII-encoded channel, such as a text file,
    /// or an e-mail.
    pub fn into_string(self) -> String {
        share_to_string(self)
    }

    /// Parse the given string into a `Share`.
    /// The `raw` string must have been generated by the
    /// `Share::to_string` method for it to succeed.
    pub fn from_string(raw: &str) -> Result<Self> {
        share_from_string(raw)
    }
}

impl IsShare for Share {
    fn get_id(&self) -> u8 {
        self.id
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_threshold(&self) -> u8 {
        self.threshold
    }

    fn get_shares_count(&self) -> Option<u8> {
        Some(self.shares_count)
    }
}