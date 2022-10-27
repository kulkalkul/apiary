use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;

pub struct GetWitnessCount;

impl Method for GetWitnessCount {
    type Params = EmptyParams;
    type Result = u64;
    const NAME: &'static str = ns!("get_witness_count");

    fn params(self) -> Self::Params { [] }
}