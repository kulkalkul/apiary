use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::HardforkVersion;

pub struct GetHardforkVersion;

impl Method for GetHardforkVersion {
    type Params = EmptyParams;
    type Result = HardforkVersion;
    const NAME: &'static str = ns!("get_hardfork_version");

    fn params(self) -> Self::Params { [] }
}