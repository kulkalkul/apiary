use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::MarketTicker;

pub struct GetTicker;

impl Method for GetTicker {
    type Params = EmptyParams;
    type Result = MarketTicker;
    const NAME: &'static str = ns!("get_ticker");

    fn params(self) -> Self::Params { [] }
}