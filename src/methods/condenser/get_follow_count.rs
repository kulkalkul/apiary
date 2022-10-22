use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{Account, FollowCount};

pub struct GetFollowCount {
    pub account: Account,
}

impl GetFollowCount {
    pub fn new<A: Into<Account>>(account: A) -> Self {
        Self { account: account.into() }
    }
}

impl Method for GetFollowCount {
    type Params = [Account; 1];
    type Result = FollowCount;
    const NAME: &'static str = ns!("get_follow_count");

    fn params(self) -> Self::Params { [self.account] }
}