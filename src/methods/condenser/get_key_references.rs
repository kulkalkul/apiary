use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::Account;

pub struct GetKeyReferences {
    pub keys: Vec<String>,
}

impl GetKeyReferences {
    pub fn new<K: ToString>(keys: &[K]) -> Self {
        let keys = keys.iter().map(ToString::to_string).collect();
        Self { keys }
    }
}

impl Method for GetKeyReferences {
    type Params = [Vec<String>; 1];
    type Result = Vec<Vec<Account>>;
    const NAME: &'static str = ns!("get_key_references");

    fn params(self) -> Self::Params { [self.keys] }
}