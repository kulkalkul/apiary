use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{Account, Follow, FollowType};

pub struct GetFollowers {
    pub account: Account,
    pub start: Option<Account>,
    pub follow_type: FollowType,
    pub limit: u16,
}

impl GetFollowers {
    pub const MAX_LIMIT: u16 = 1_000;

    pub fn new<A: Into<Account>>(
        account: A,
        start: Option<A>,
        follow_type: FollowType,
        limit: u16,
    ) -> Self {
        Self {
            account: account.into(),
            start: start.map(Into::into),
            follow_type,
            limit,
        }
    }

    pub fn of_max<A: Into<Account>>(account: A, start: A, follow_type: FollowType) -> Self {
        Self::new(account, Some(start), follow_type, Self::MAX_LIMIT)
    }
    pub fn of_blog_max<A: Into<Account>>(account: A, start: A) -> Self {
        Self::of_max(account, start, FollowType::Blog)
    }
    pub fn of_ignore_max<A: Into<Account>>(account: A, start: A) -> Self {
        Self::of_max(account, start, FollowType::Ignore)
    }
    pub fn of_all<A: Into<Account>>(account: A, follow_type: FollowType, limit: u16) -> Self {
        Self::new(account, None, follow_type, limit)
    }
    pub fn of_blog_all<A: Into<Account>>(account: A, limit: u16) -> Self {
        Self::of_all(account, FollowType::Blog, limit)
    }
    pub fn of_ignore_all<A: Into<Account>>(account: A, limit: u16) -> Self {
        Self::of_all(account, FollowType::Ignore, limit)
    }
    pub fn of_all_max<A: Into<Account>>(account: A, follow_type: FollowType) -> Self {
        Self::new(account, None, follow_type, Self::MAX_LIMIT)
    }
    pub fn of_blog_all_max<A: Into<Account>>(account: A) -> Self {
        Self::of_all_max(account, FollowType::Blog)
    }
    pub fn of_ignore_all_max<A: Into<Account>>(account: A) -> Self {
        Self::of_all_max(account, FollowType::Ignore)
    }
}

impl Method for GetFollowers {
    type Params = (Account, Option<Account>, FollowType, u16);
    type Result = Vec<Follow>;
    const NAME: &'static str = ns!("get_followers");

    fn params(self) -> Self::Params { (self.account, self.start, self.follow_type, self.limit) }
}