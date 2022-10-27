use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::Version;

pub struct GetVersion;

impl Method for GetVersion {
    type Params = EmptyParams;
    type Result = Version;
    const NAME: &'static str = ns!("get_version");

    fn params(self) -> Self::Params { [] }
}