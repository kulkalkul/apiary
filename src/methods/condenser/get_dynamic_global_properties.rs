use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::DynamicGlobalProperties;

pub struct GetDynamicGlobalProperties;

impl Method for GetDynamicGlobalProperties {
    type Params = EmptyParams;
    type Result = DynamicGlobalProperties;
    const NAME: &'static str = ns!("get_dynamic_global_properties");

    fn params(self) -> Self::Params { [] }
}