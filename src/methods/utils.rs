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

// TODO: Parse it from string, rename to LegacyAsset
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
pub struct VestingDelegationExpirationObjectId(u32);

impl ObjectId for VestingDelegationExpirationObjectId {
    fn new(object_id: u32) -> Self { Self(object_id) }
    fn value(&self) -> u32 { self.0 }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct FeedHistoryObjectId(u32);

impl ObjectId for FeedHistoryObjectId {
    fn new(object_id: u32) -> Self { Self(object_id) }
    fn value(&self) -> u32 { self.0 }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct RewardFundObjectId(u32);

impl ObjectId for RewardFundObjectId {
    fn new(object_id: u32) -> Self { Self(object_id) }
    fn value(&self) -> u32 { self.0 }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct VestingDelegationObjectId(u32);

impl ObjectId for VestingDelegationObjectId {
    fn new(object_id: u32) -> Self { Self(object_id) }
    fn value(&self) -> u32 { self.0 }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct ProposalId(pub u32);

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FollowType {
    Blog,
    Ignore,
}

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

// Maybe use a lower level repr like [u8; 20]?
#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct BlockId(pub String);

#[serde_as]
#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DynamicGlobalProperties {
    pub head_block_number: u32,
    pub head_block_id: BlockId,
    pub time: NaiveDateTime,
    pub current_witness: Account,
    pub total_pow: u64,
    pub num_pow_witnesses: u32,
    pub virtual_supply: Asset,
    pub current_supply: Asset,
    pub init_hbd_supply: Asset,
    pub current_hbd_supply: Asset,
    pub total_vesting_fund_hive: Asset,
    pub total_vesting_shares: Asset,
    pub total_reward_fund_hive: Asset,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub total_reward_shares2: u128,
    pub pending_rewarded_vesting_shares: Asset,
    pub pending_rewarded_vesting_hive: Asset,
    pub hbd_interest_rate: u16,
    pub hbd_print_rate: u16,
    pub maximum_block_size: u32,
    pub required_actions_partition_percent: u16,
    pub current_aslot: u64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub recent_slots_filled: u128,
    pub participation_count: u8,
    pub last_irreversible_block_num: u32,
    pub vote_power_reserve_rate: u32,
    pub delegation_return_period: u32,
    pub reverse_auction_seconds: u64,
    pub available_account_subsidies: i64,
    pub hbd_stop_percent: u16,
    pub hbd_start_percent: u16,
    pub next_maintenance_time: NaiveDateTime,
    pub last_budget_time: NaiveDateTime,
    pub next_daily_maintenance_time: NaiveDateTime,
    pub content_reward_percent: u16,
    pub vesting_reward_percent: u16,
    pub proposal_fund_percent: u16,
    pub dhf_interval_ledger: Asset,
    pub downvote_pool_percent: u16,
    pub current_remove_threshold: i16,
    pub early_voting_seconds: u64,
    pub mid_voting_seconds: u64,
    pub max_consecutive_recurrent_transfer_failures: u8,
    pub max_recurrent_transfer_end_date: u16,
    pub min_recurrent_transfers_recurrence: u8,
    pub max_open_recurrent_transfers: u16,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExpiringVestingDelegation {
    pub id: VestingDelegationExpirationObjectId,
    pub delegator: Account,
    pub vesting_shares: Asset,
    pub expiration: NaiveDateTime,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FeedHistory {
    pub id: FeedHistoryObjectId,
    pub current_max_history: Price,
    pub current_median_history: Price,
    pub current_min_history: Price,
    pub market_median_history: Price,
    pub price_history: Vec<Price>,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FollowCount {
    pub account: Account,
    pub follower_count: u32,
    pub following_count: u32,
}

#[serde_as]
#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Follow {
    pub follower: Account,
    pub following: Account,
    pub what: Vec<FollowType>,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct HardforkVersion(pub String);

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NextScheduledHardfork {
    pub hf_version: HardforkVersion,
    pub live_time: NaiveDateTime,
}

#[derive(Serialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RewardFundType {
    Post,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CurveId {
    Quadratic,
    BoundedCuration,
    Linear,
    SquareRoot,
    ConvergentLinear,
    ConvergentSquareRoot,
}

#[serde_as]
#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RewardFund {
    pub id: RewardFundObjectId,
    pub name: String,
    pub reward_balance: Asset,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub recent_claims: u128,
    pub last_update: NaiveDateTime,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub content_constant: u128,
    pub percent_curation_rewards: u16,
    pub percent_content_rewards: u16,
    pub author_reward_curve: CurveId,
    pub curation_reward_curve: CurveId,
}

#[serde_as]
#[derive(Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct MarketTicker {
    #[serde_as(as = "DisplayFromStr")]
    pub latest: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub lowest_ask: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub highest_bid: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub percent_change: f64,
    pub hive_volume: Asset,
    pub hbd_volume: Asset,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TrendingTag {
    pub name: String,
    pub comments: u64,
    pub top_posts: u64,
    pub total_payouts: Asset,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Version {
    pub blockchain_version: HardforkVersion,
    pub hive_revision: String,
    pub fc_revision: String,
    pub chain_id: String,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VestingDelegation {
    pub id: VestingDelegationObjectId,
    pub delegator: Account,
    pub delegatee: Account,
    pub vesting_shares: Asset,
    pub min_delegation_time: NaiveDateTime,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Volume {
    pub hive_volume: Asset,
    pub hbd_volume: Asset,
}