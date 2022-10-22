use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{Account, AccountReputation};

pub struct GetAccountReputations {
    pub account_lower_bound: Account,
    pub limit: u16,
}

impl GetAccountReputations {
    pub fn new<A: Into<Account>>(account_lower_bound: A, limit: u16) -> Self {
        Self {
            account_lower_bound: account_lower_bound.into(),
            limit,
        }
    }
}

impl Method for GetAccountReputations {
    type Params = (Account, u16);
    type Result = Vec<AccountReputation>;
    const NAME: &'static str = ns!("get_account_reputations");

    fn params(self) -> Self::Params { (self.account_lower_bound, self.limit) }
}