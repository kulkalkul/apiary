use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::Price;

pub struct GetCurrentMedianHistoryPrice;

impl Method for GetCurrentMedianHistoryPrice {
    type Params = EmptyParams;
    type Result = Price;
    const NAME: &'static str = ns!("get_current_median_history_price");

    fn params(self) -> Self::Params { [] }
}