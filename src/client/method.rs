use serde::{Serialize};
use serde::de::DeserializeOwned;
use serde_json::json;

pub trait Method: Sized {
    type Params: Serialize;
    type Result: DeserializeOwned;
    const NAME: &'static str;

    fn params(self) -> Self::Params;
    fn build(self, id: u64) -> String {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": Self::NAME,
            "params": self.params(),
        }).to_string()
    }
}

// TODO: Change into struct
pub type EmptyParams = [(); 0];

// TODO: Maybe implement a Params deserializer struct