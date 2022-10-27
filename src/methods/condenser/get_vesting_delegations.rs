use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{Account, VestingDelegation};

pub struct GetVestingDelegations {
    pub delegator_account: Account,
    pub start_account: Option<Account>,
    pub limit: u16,
}

impl GetVestingDelegations {
    pub const MAX_LIMIT: u16 = 1_000;

    pub fn new<A: Into<Account>>(
        delegator_account: A,
        start_account: Option<A>,
        limit: u16,
    ) -> Self {
        Self {
            delegator_account: delegator_account.into(),
            start_account: start_account.map(Into::into),
            limit,
        }
    }
    pub fn of_all<A: Into<Account>>(delegator_account: A, limit: u16) -> Self {
        Self::new(delegator_account, None, limit)
    }
    pub fn of_max<A: Into<Account>>(delegator_account: A, start_account: A) -> Self {
        Self::new(delegator_account, Some(start_account), Self::MAX_LIMIT)
    }
    pub fn of_all_max<A: Into<Account>>(delegator_account: A) -> Self {
        Self::of_all(delegator_account, Self::MAX_LIMIT)
    }
}

impl Method for GetVestingDelegations {
    type Params = (Account, Option<Account>, u16);
    type Result = Vec<VestingDelegation>;
    const NAME: &'static str = ns!("get_vesting_delegations");

    fn params(self) -> Self::Params { (self.delegator_account, self.start_account, self.limit) }
}