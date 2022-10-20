mod find_proposals;
mod get_account_count;
mod get_account_reputations;
mod get_active_votes;
mod get_active_witnesses;
mod get_blog_entries;
mod get_chain_properties;
mod get_current_median_history_price;

pub use find_proposals::FindProposals;
pub use get_account_count::GetAccountCount;
pub use get_account_reputations::GetAccountReputations;
pub use get_active_votes::GetActiveVotes;
pub use get_active_witnesses::GetActiveWitnesses;
pub use get_blog_entries::GetBlogEntries;
pub use get_chain_properties::GetChainProperties;
pub use get_current_median_history_price::GetCurrentMedianHistoryPrice;

#[macro_export]
macro_rules! condenser_namespace {
    ($s: literal) => (::core::concat!("condenser_api.", $s));
}

pub use condenser_namespace as ns;