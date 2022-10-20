pub mod client;
pub mod methods;

#[cfg(test)]
mod tests {
    use tokio;
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

        assert_eq!(account_reputations.len(), limit);

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
}