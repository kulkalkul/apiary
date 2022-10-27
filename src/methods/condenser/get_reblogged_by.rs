use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::Account;

pub struct GetRebloggedBy {
    pub author: Account,
    pub permlink: String,
}

impl GetRebloggedBy {
    pub fn new<A: Into<Account>, S: ToString>(author: A, permlink: S) -> Self {
        Self {
            author: author.into(),
            permlink: permlink.to_string(),
        }
    }
}

impl Method for GetRebloggedBy {
    type Params = (Account, String);
    type Result = Vec<Account>;
    const NAME: &'static str = ns!("get_reblogged_by");

    fn params(self) -> Self::Params { (self.author, self.permlink) }
}