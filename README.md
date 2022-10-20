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
- [ ] condenser_api
  - [ ] broadcast_block
  - [ ] broadcast_transaction
  - [ ] broadcast_transaction_synchronous
  - [x] find_proposals
  - [ ] find_recurrent_transfers
  - [x] get_account_count
  - [ ] get_account_history
  - [x] get_account_reputations
  - [ ] get_accounts
  - [x] get_active_votes
  - [x] get_active_witnesses
  - [ ] get_block
  - [ ] get_block_header
  - [ ] get_blog
  - [x] get_blog_entries
  - [x] get_chain_properties
  - [ ] get_comment_discussions_by_payout
  - [ ] get_config
  - [ ] get_content
  - [ ] get_content_replies
  - [x] get_current_median_history_price

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