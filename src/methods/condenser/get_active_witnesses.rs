use crate::client::{EmptyParams, Method};
use crate::methods::condenser::ns;
use crate::methods::utils::Account;

pub struct GetActiveWitnesses;

impl Method for GetActiveWitnesses {
    type Params = EmptyParams;
    type Result = Vec<Account>;
    const NAME: &'static str = ns!("get_active_witnesses");

    fn params(self) -> Self::Params { [] }
}