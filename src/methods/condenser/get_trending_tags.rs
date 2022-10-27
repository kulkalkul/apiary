use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::TrendingTag;

pub struct GetTrendingTags {
    pub tag: Option<String>,
    pub limit: u8,
}

impl GetTrendingTags {
    pub const MAX_LIMIT: u8 = 100;

    pub fn new<S: ToString>(tag: Option<S>, limit: u8) -> Self {
        Self {
            tag: tag.as_ref().map(ToString::to_string),
            limit,
        }
    }
    // not using new because new requires S to be defined
    pub fn all(limit: u8) -> Self {
        Self {
            tag: None,
            limit,
        }
    }
    pub fn all_max() -> Self {
        Self::all(Self::MAX_LIMIT)
    }
    pub fn of<S: ToString>(tag: S, limit: u8) -> Self {
        Self::new(Some(tag), limit)
    }
    pub fn of_max<S: ToString>(tag: S) -> Self {
        Self::of(tag, Self::MAX_LIMIT)
    }
}

impl Method for GetTrendingTags {
    type Params = (Option<String>, u8);
    type Result = Vec<TrendingTag>;
    const NAME: &'static str = ns!("get_trending_tags");

    fn params(self) -> Self::Params { (self.tag, self.limit) }
}
