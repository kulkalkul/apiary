use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::ChainProperties;

pub struct GetChainProperties;

impl Method for GetChainProperties {
    type Params = EmptyParams;
    type Result = ChainProperties;
    const NAME: &'static str = ns!("get_chain_properties");

    fn params(self) -> Self::Params { [] }
}