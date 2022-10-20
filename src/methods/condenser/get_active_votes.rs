use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{Account, VoteState};

pub struct GetActiveVotes {
    pub author: Account,
    pub permlink: String,
}

impl GetActiveVotes {
    pub fn new<A: Into<Account>, P: Into<String>>(author: A, permlink: P) -> Self {
        Self {
            author: author.into(),
            permlink: permlink.into(),
        }
    }
}

impl Method for GetActiveVotes {
    type Params = (Account, String);
    type Result = Vec<VoteState>;
    const NAME: &'static str = ns!("get_active_votes");

    fn params(self) -> Self::Params { (self.author, self.permlink) }
}