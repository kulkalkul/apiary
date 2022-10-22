use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, PickFirst, serde_as};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct Account(pub String);

impl Account {
    pub fn new<A: Into<Self>>(account: A) -> Self {
        account.into()
    }
}

impl From<String> for Account {
    fn from(account: String) -> Self {
        Self(account)
    }
}
impl From<&str> for Account {
    fn from(account: &str) -> Self {
        Self(account.to_string())
    }
}

// TODO: Parse it from string
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct Asset(pub String);

// chainbase/util/object_id.hpp
pub trait ObjectId: Sized {
    fn new(object_id: u32) -> Self;
    fn value(&self) -> u32;
    fn ids_from_slice(ids: &[u32]) -> Vec<Self> {
        ids.iter().copied().map(Self::new).collect()
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct ProposalObjectId(u32);

impl ObjectId for ProposalObjectId {
    fn new(object_id: u32) -> Self { Self(object_id) }
    fn value(&self) -> u32 { self.0 }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct ProposalId(pub u32);

#[serde_as]
#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Proposal {
    pub id: ProposalObjectId,
    pub proposal_id: ProposalId,
    pub creator: Account,
    pub receiver: Account,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub daily_pay: Asset,
    pub subject: String,
    pub permlink: String,
    #[serde_as(as = "DisplayFromStr")]
    pub total_votes: u64,
}

#[serde_as]
#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AccountReputation {
    pub account: Account,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub reputation: i64,
}

#[serde_as]
#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VoteState {
    pub voter: Account,
    pub weight: u64,
    pub rshares: i64,
    pub percent: i16,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub reputation: i64,
    pub time: NaiveDateTime,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BlogEntry {
    pub author: Account,
    pub permlink: String,
    pub blog: String,
    pub reblogged_on: NaiveDateTime,
    pub entry_id: u64,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ChainProperties {
    pub account_creation_fee: Asset,
    pub maximum_block_size: u32,
    pub hbd_interest_rate: u16,
    pub account_subsidy_budget: i32,
    pub account_subsidy_decay: u32,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Price {
    pub base: Asset,
    pub quote: Asset,
}

\ No newline at end of file
