use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{Account, BlogEntry};

pub struct GetBlogEntries {
    pub account: Account,
    pub start_entry_id: u64,
    pub limit: u16,
}

impl GetBlogEntries {
    pub const START_LATEST: u64 = 0;
    pub const LIMIT_MAX: u16 = 500;

    pub fn new<A: Into<Account>>(account: A, start_entry_id: u64, limit: u16) -> Self {
        Self {
            account: account.into(),
            start_entry_id,
            limit,
        }
    }
    pub fn start_max<A: Into<Account>>(account: A, start_entry_id: u64) -> Self {
        Self::new(account, start_entry_id, Self::LIMIT_MAX)
    }
    pub fn latest_max<A: Into<Account>>(account: A) -> Self {
        Self::new(account, Self::START_LATEST, Self::LIMIT_MAX)
    }
    pub fn latest<A: Into<Account>>(account: A, limit: u16) -> Self {
        Self::new(account, Self::START_LATEST, limit)
    }
}

impl Method for GetBlogEntries {
    type Params = (Account, u64, u16);
    type Result = Vec<BlogEntry>;
    const NAME: &'static str = ns!("get_blog_entries");

    fn params(self) -> Self::Params { ( self.account, self.start_entry_id, self.limit ) }
}