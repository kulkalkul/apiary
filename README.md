# Apiary - Hive API Utility

# **WIP**

## Todo
- Full API Coverage
- Follow Rust API Guidelines:
  https://rust-lang.github.io/api-guidelines/about.html
  - Implement Display for utils
- Extract utils into better structured folders
- Add high-level helpers if possible, like helper for querying with limits

## API Coverage
- <details>
  <summary>condenser_api <b>[29/89]</b></summary>

  - [ ] broadcast_transaction
  - [ ] broadcast_transaction_synchronous
  - [x] find_proposals
  - [ ] find_rc_accounts
  - [ ] find_recurrent_transfers
  - [x] get_account_count
  - [ ] get_account_history
  - [ ] get_account_references
  - [x] get_account_reputations
  - [ ] get_accounts
  - [x] get_active_votes
  - [x] get_active_witnesses
  - [ ] get_block
  - [ ] get_block_header
  - [ ] get_blog
  - [x] get_blog_entries
  - [x] get_chain_properties
  - [ ] get_collateralized_conversion_requests
  - [ ] get_comment_discussions_by_payout
  - [ ] get_config
  - [ ] get_content
  - [ ] get_content_replies
  - [ ] get_conversion_requests
  - [x] get_current_median_history_price
  - [ ] get_discussions_by_active
  - [ ] get_discussions_by_author_before_date
  - [ ] get_discussions_by_blog
  - [ ] get_discussions_by_cashout
  - [ ] get_discussions_by_children
  - [ ] get_discussions_by_comments
  - [ ] get_discussions_by_created
  - [ ] get_discussions_by_feed
  - [ ] get_discussions_by_hot
  - [ ] get_discussions_by_promoted
  - [ ] get_discussions_by_trending
  - [ ] get_discussions_by_votes
  - [x] get_dynamic_global_properties
  - [ ] get_escrow
  - [x] get_expiring_vesting_delegations
  - [ ] get_feed
  - [ ] get_feed_entries
  - [x] get_feed_history
  - [x] get_follow_count
  - [x] get_followers
  - [x] get_following
  - [x] get_hardfork_version
  - [x] get_key_references
  - [ ] get_market_history
  - [ ] get_market_history_buckets
  - [x] get_next_scheduled_hardfork
  - [ ] get_open_orders
  - [ ] get_ops_in_block
  - [ ] get_order_book
  - [ ] get_owner_history
  - [ ] get_post_discussions_by_payout
  - [ ] get_potential_signatures
  - [x] get_reblogged_by
  - [ ] get_recent_trades
  - [ ] get_recovery_request
  - [ ] get_replies_by_last_update
  - [ ] get_required_signatures
  - [x] get_reward_fund
  - [ ] get_savings_withdraw_from
  - [ ] get_savings_withdraw_to
  - [ ] get_tags_used_by_author
  - [x] get_ticker
  - [ ] get_trade_history
  - [ ] get_transaction
  - [ ] get_transaction_hex
  - [x] get_trending_tags
  - [x] get_version
  - [x] get_vesting_delegations
  - [x] get_volume
  - [ ] get_withdraw_routes
  - [ ] get_witness_by_account
  - [x] get_witness_count
  - [ ] get_witness_schedule
  - [ ] get_witnesses
  - [ ] get_witnesses_by_vote
  - [x] is_known_transaction
  - [ ] list_proposal_votes
  - [ ] list_proposals
  - [ ] list_rc_accounts
  - [ ] list_direct_rc_delegations
  - [ ] lookup_account_names
  - [x] lookup_accounts
  - [x] lookup_witness_accounts
  - [ ] verify_account_authority
  - [ ] verify_authority
</details>

- <details>
  <summary>account_by_key_api <b>[0/1]</b></summary>
  
  - [ ] get_key_references
</details>

- <details>
  <summary>bridge_api <b>[0/18]</b></summary>

  - [ ] account_notifications
  - [ ] does_user_follow_any_lists
  - [ ] get_account_posts
  - [ ] get_community
  - [ ] get_community_context
  - [ ] get_discussion
  - [ ] get_follow_list
  - [ ] get_payout_stats
  - [ ] get_post
  - [ ] get_post_header
  - [ ] get_profile
  - [ ] get_ranked_posts
  - [ ] get_relationship_between_accounts
  - [ ] list_all_subscriptions
  - [ ] list_communities
  - [ ] list_community_roles
  - [ ] list_pop_communities
  - [ ] list_subscribers
</details>

##
<details>
  <summary>
  Deprecated & Not Supported (Feel free to open an issue)
  </summary>
  
  - broadcast_block **(removed/hf26)**
  - get_account_bandwidth **(removed/hf24)**
  - get_account_votes **(removed?)**
  - get_blog_authors **(hivemind not supporting?)**
  - get_state **(deprecated)**

</details>

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.