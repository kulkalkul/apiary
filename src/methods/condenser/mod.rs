mod find_proposals;
mod get_account_count;
mod get_account_reputations;
mod get_active_votes;
mod get_active_witnesses;
mod get_blog_entries;
mod get_chain_properties;
mod get_current_median_history_price;
mod get_dynamic_global_properties;
mod get_expiring_vesting_delegations;
mod get_feed_history;
mod get_follow_count;
mod get_followers;

pub use find_proposals::FindProposals;
pub use get_account_count::GetAccountCount;
pub use get_account_reputations::GetAccountReputations;
pub use get_active_votes::GetActiveVotes;
pub use get_active_witnesses::GetActiveWitnesses;
pub use get_blog_entries::GetBlogEntries;
pub use get_chain_properties::GetChainProperties;
pub use get_current_median_history_price::GetCurrentMedianHistoryPrice;
pub use get_dynamic_global_properties::GetDynamicGlobalProperties;
pub use get_expiring_vesting_delegations::GetExpiringVestingDelegations;
pub use get_feed_history::GetFeedHistory;
pub use get_follow_count::GetFollowCount;
pub use get_followers::GetFollowers;


#[macro_export]
macro_rules! condenser_namespace {
    ($s: literal) => (::core::concat!("condenser_api.", $s));
}

pub use condenser_namespace as ns;
