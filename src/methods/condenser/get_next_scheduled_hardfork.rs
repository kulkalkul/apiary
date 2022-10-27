use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::NextScheduledHardfork;

pub struct GetNextScheduledHardfork;

impl Method for GetNextScheduledHardfork {
    type Params = EmptyParams;
    type Result = NextScheduledHardfork;
    const NAME: &'static str = ns!("get_next_scheduled_hardfork");

    fn params(self) -> Self::Params { [] }
}