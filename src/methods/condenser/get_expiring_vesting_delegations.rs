use chrono::NaiveDateTime;
use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{Account, ExpiringVestingDelegation};

pub struct GetExpiringVestingDelegations {
    pub account: Account,
    pub after: NaiveDateTime,
}

impl GetExpiringVestingDelegations {
    pub fn new<A: Into<Account>>(account: A, after: NaiveDateTime) -> Self {
        Self {
            account: account.into(),
            after,
        }
    }
}

impl Method for GetExpiringVestingDelegations {
    type Params = (Account, NaiveDateTime);
    type Result = Vec<ExpiringVestingDelegation>;
    const NAME: &'static str = ns!("get_expiring_vesting_delegations");

    fn params(self) -> Self::Params { (self.account, self.after) }
}