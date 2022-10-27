pub mod client;
pub mod methods;

#[cfg(test)]
mod tests {
    use tokio;
    use chrono::{Utc};
    use crate::client::{HiveClient, HiveNodes};
    use pretty_assertions::{assert_eq};

    fn client() -> HiveClient {
        HiveClient::new(HiveNodes::Deathwing)
    }

    #[tokio::test]
    async fn find_proposals() {
        use crate::methods::condenser::FindProposals;
        use crate::methods::utils::ObjectId;

        let hive_client = client();
        let ids = &[0, 1, 2];

        let proposals = hive_client
            .method()
            .send(FindProposals::new(ids))
            .await
            .unwrap();

        let proposal_ids = proposals.iter().map(|x| x.id.value()).collect::<Vec<u32>>();

        assert_eq!(ids, proposal_ids.as_slice());
    }

    #[tokio::test]
    async fn get_account_count() {
        use crate::methods::condenser::GetAccountCount;

        let hive_client = client();

        let account_count = hive_client
            .method()
            .send(GetAccountCount)
            .await
            .unwrap();

        assert!(account_count > 0)
    }

    #[tokio::test]
    async fn get_account_reputations() {
        use crate::methods::condenser::GetAccountReputations;

        let hive_client = client();

        let account = "yokunjon";
        let limit = 10;

        let account_reputations = hive_client
            .method()
            .send(GetAccountReputations::new(account, limit))
            .await
            .unwrap();

        assert_eq!(account_reputations.len(), limit as usize);

        let account_exists_response = account_reputations
            .iter()
            .any(|x| x.account.0 == account);

        assert!(account_exists_response);
    }

    #[tokio::test]
    async fn get_active_votes() {
        use crate::methods::condenser::GetActiveVotes;

        let hive_client = client();

        let account = "blocktrades";
        let permlink = "15th-update-of-2022-new-hive-hf26-release-candidate-3-released";

        let active_votes = hive_client
            .method()
            .send(GetActiveVotes::new(account, permlink))
            .await
            .unwrap();

        assert!(active_votes.len() > 0);
    }

    #[tokio::test]
    async fn get_active_witnesses() {
        use crate::methods::condenser::GetActiveWitnesses;

        let hive_client = client();

        let active_witnesses  = hive_client
            .method()
            .send(GetActiveWitnesses)
            .await
            .unwrap();

        assert!(active_witnesses.len() > 0);
    }

    #[tokio::test]
    async fn get_blog_entries() {
        use crate::methods::condenser::GetBlogEntries;

        let hive_client = client();

        let account = "arcange";
        let limit = 10;

        let blog_entries = hive_client
            .method()
            .send(GetBlogEntries::latest(account, limit))
            .await
            .unwrap();

        assert_eq!(blog_entries.len(), limit as usize);
    }

    #[tokio::test]
    async fn get_chain_properties() {
        use crate::methods::condenser::GetChainProperties;

        let hive_client = client();

        let chain_properties = hive_client
            .method()
            .send(GetChainProperties)
            .await
            .unwrap();

        assert!(chain_properties.maximum_block_size > 0);
    }

    #[tokio::test]
    async fn get_current_median_history_price() {
        use crate::methods::condenser::GetCurrentMedianHistoryPrice;

        let hive_client = client();

        let _current_median_history_price = hive_client
            .method()
            .send(GetCurrentMedianHistoryPrice)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_dynamic_global_properties() {
        use crate::methods::condenser::GetDynamicGlobalProperties;

        let hive_client = client();

        let _dynamic_global_properties = hive_client
            .method()
            .send(GetDynamicGlobalProperties)
            .await
            .unwrap();
    }

    // Not a good test
    #[tokio::test]
    async fn get_expiring_vesting_delegations() {
        use crate::methods::condenser::GetExpiringVestingDelegations;

        let hive_client = client();

        let account = "themarkymark";
        let after = Utc::now().naive_utc();

        let _expiring_vesting_delegations = hive_client
            .method()
            .send(GetExpiringVestingDelegations::new(account, after))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_feed_history() {
        use crate::methods::condenser::GetFeedHistory;

        let hive_client = client();

        let _feed_history = hive_client
            .method()
            .send(GetFeedHistory)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_follow_count() {
        use crate::methods::condenser::GetFollowCount;

        let hive_client = client();

        let account = "crimsonclad";

        let get_follow_count = hive_client
            .method()
            .send(GetFollowCount::new(account))
            .await
            .unwrap();

        assert_eq!(get_follow_count.account.0, account);
    }

    #[tokio::test]
    async fn get_followers() {
        use crate::methods::condenser::GetFollowers;
        use crate::methods::utils::FollowType;

        let hive_client = client();

        let account = "deathwing";
        let start = Some("yokunjon");
        let follow_type = FollowType::Blog;
        let limit = 2;

        let followers = hive_client
            .method()
            .send(GetFollowers::new(account, start, follow_type, limit))
            .await
            .unwrap();

        assert_eq!(followers.len(), limit as usize);
    }

    #[tokio::test]
    async fn get_following() {
        use crate::methods::condenser::GetFollowers;
        use crate::methods::utils::FollowType;

        let hive_client = client();

        let account = "deathwing";
        let start = Some("yokunjon");
        let follow_type = FollowType::Blog;
        let limit = 2;

        let following = hive_client
            .method()
            .send(GetFollowers::new(account, start, follow_type, limit))
            .await
            .unwrap();

        assert_eq!(following.len(), limit as usize);
    }

    #[tokio::test]
    async fn get_hardfork_version() {
        use crate::methods::condenser::GetHardforkVersion;

        let hive_client = client();

        let _hardfork_version = hive_client
            .method()
            .send(GetHardforkVersion)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_key_references() {
        use crate::methods::condenser::GetKeyReferences;
        use crate::methods::utils::Account;

        let hive_client = client();

        let keys = &["STM6vJmrwaX5TjgTS9dPH8KsArso5m91fVodJvv91j7G765wqcNM9"];

        let accounts = hive_client
            .method()
            .send(GetKeyReferences::new(keys))
            .await
            .unwrap();

        assert_eq!(accounts, vec![vec![Account::new("hiveio")]]);
    }

    #[tokio::test]
    async fn get_next_scheduled_hardfork() {
        use crate::methods::condenser::GetNextScheduledHardfork;

        let hive_client = client();

        let _next_scheduled_hardfork = hive_client
            .method()
            .send(GetNextScheduledHardfork)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_reblogged_by() {
        use crate::methods::condenser::GetRebloggedBy;

        let hive_client = client();

        let author = "hiveio";
        let permlink = "the-evolution-of-hive-hardfork-26";

        let reblogged_by = hive_client
            .method()
            .send(GetRebloggedBy::new(author, permlink))
            .await
            .unwrap();

        assert!(reblogged_by.len() > 0);
    }

    #[tokio::test]
    async fn get_reward_fund() {
        use crate::methods::condenser::GetRewardFund;
        use crate::methods::utils::RewardFundType;

        let hive_client = client();

        let _reward_fund = hive_client
            .method()
            .send(GetRewardFund::new(RewardFundType::Post))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_ticker() {
        use crate::methods::condenser::GetTicker;

        let hive_client = client();

        let _ticker = hive_client
            .method()
            .send(GetTicker)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_trending_tags() {
        use crate::methods::condenser::GetTrendingTags;

        let hive_client = client();

        let trending_tags = hive_client
            .method()
            .send(GetTrendingTags::all_max())
            .await
            .unwrap();

        assert!(trending_tags.len() > 0);
    }

    #[tokio::test]
    async fn get_version() {
        use crate::methods::condenser::GetVersion;

        let hive_client = client();

        let _version = hive_client
            .method()
            .send(GetVersion)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn get_vesting_delegations() {
        use crate::methods::condenser::GetVestingDelegations;

        let hive_client = client();

        let delegator_account = "deathwing";

        let _vesting_delegations = hive_client
            .method()
            .send(GetVestingDelegations::of_all_max(delegator_account))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_volume() {
        use crate::methods::condenser::GetVolume;

        let hive_client = client();

        let _volume = hive_client
            .method()
            .send(GetVolume)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_witness_count() {
        use crate::methods::condenser::GetWitnessCount;

        let hive_client = client();

        let witness_count = hive_client
            .method()
            .send(GetWitnessCount)
            .await
            .unwrap();

        assert!(witness_count > 0);
    }

    #[tokio::test]
    async fn is_known_transaction() {
        use crate::methods::condenser::IsKnownTransaction;

        let hive_client = client();

        let is_known_transaction = hive_client
            .method()
            .send(IsKnownTransaction::new("0000000000000000000000000000000000000000"))
            .await
            .unwrap();

        assert_eq!(is_known_transaction, false);
    }

    #[tokio::test]
    async fn lookup_accounts() {
        use crate::methods::condenser::LookupAccounts;

        let hive_client = client();

        let lower_bound_name = "yokunjon";
        let limit = 10;

        let accounts = hive_client
            .method()
            .send(LookupAccounts::new(lower_bound_name, limit))
            .await
            .unwrap();

        assert_eq!(accounts.len(), limit as usize);
    }

    #[tokio::test]
    async fn lookup_witness_accounts() {
        use crate::methods::condenser::LookupWitnessAccounts;

        let hive_client = client();

        let lower_bound_name = "deathwing";
        let limit = 10;

        let witness_accounts = hive_client
            .method()
            .send(LookupWitnessAccounts::new(lower_bound_name, limit))
            .await
            .unwrap();

        assert_eq!(witness_accounts.len(), limit as usize);
    }
}