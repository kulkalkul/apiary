use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::Account;

pub struct LookupAccounts {
    pub lower_bound_name: Account,
    pub limit: u16,
}

impl LookupAccounts {
    pub const MAX_LIMIT: u16 = 1_000;

    pub fn new<A: Into<Account>>(lower_bound_name: A, limit: u16) -> Self {
        Self {
            lower_bound_name: lower_bound_name.into(),
            limit,
        }
    }
    pub fn max<A: Into<Account>>(lower_bound_name: A) -> Self {
        Self::new(lower_bound_name, Self::MAX_LIMIT)
    }
}

impl Method for LookupAccounts {
    type Params = (Account, u16);
    type Result = Vec<Account>;
    const NAME: &'static str = ns!("lookup_accounts");

    fn params(self) -> Self::Params { (self.lower_bound_name, self.limit) }
}