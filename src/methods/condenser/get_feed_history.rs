use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::FeedHistory;

pub struct GetFeedHistory;

impl Method for GetFeedHistory {
    type Params = EmptyParams;
    type Result = FeedHistory;
    const NAME: &'static str = ns!("get_feed_history");

    fn params(self) -> Self::Params { [] }
}