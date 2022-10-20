use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;

pub struct GetAccountCount;

impl Method for GetAccountCount {
    type Params = EmptyParams;
    type Result = u64;
    const NAME: &'static str = ns!("get_account_count");

    fn params(self) -> Self::Params { [] }
}