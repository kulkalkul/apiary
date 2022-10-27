use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::Volume;

pub struct GetVolume;

impl Method for GetVolume {
    type Params = EmptyParams;
    type Result = Volume;
    const NAME: &'static str = ns!("get_volume");

    fn params(self) -> Self::Params { [] }
}