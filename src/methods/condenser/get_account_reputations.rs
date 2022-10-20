use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{Account, AccountReputation};

pub struct GetAccountReputations {
    pub account_lower_bound: Account,
    pub limit: usize,
}

impl GetAccountReputations {
    pub fn new<A: Into<Account>>(account_lower_bound: A, limit: usize) -> Self {
        Self {
            account_lower_bound: account_lower_bound.into(),
            limit,
        }
    }
}

impl Method for GetAccountReputations {
    type Params = (Account, usize);
    type Result = Vec<AccountReputation>;
    const NAME: &'static str = ns!("get_account_reputations");

    fn params(self) -> Self::Params { (self.account_lower_bound, self.limit) }
}