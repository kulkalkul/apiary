use crate::client::Method;
use crate::methods::condenser::ns;

pub struct IsKnownTransaction {
    pub transaction: String,
}

impl IsKnownTransaction {
    pub fn new<S: ToString>(transaction: S) -> Self {
        Self { transaction: transaction.to_string() }
    }
}

impl Method for IsKnownTransaction {
    type Params = [String; 1];
    type Result = bool;
    const NAME: &'static str = ns!("is_known_transaction");

    fn params(self) -> Self::Params { [self.transaction] }
}