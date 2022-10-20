use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::CurrentMedianHistoryPrice;

pub struct GetCurrentMedianHistoryPrice;

impl Method for GetCurrentMedianHistoryPrice {
    type Params = EmptyParams;
    type Result = CurrentMedianHistoryPrice;
    const NAME: &'static str = ns!("get_current_median_history_price");

    fn params(self) -> Self::Params { [] }
}