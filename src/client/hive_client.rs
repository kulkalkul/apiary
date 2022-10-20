use chrono::Utc;
use reqwest::{Client};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use crate::client::method::Method;

#[derive(Debug)]
pub struct HiveNode(String);

impl HiveNode {
    pub fn new<T: ToString>(node: T) -> Self {
        Self::from(node)
    }
}

#[derive(Debug)]
pub enum HiveNodes {
    HiveBlog,
    Deathwing,
    Arcange,
    Emrebeyler,
    OpenHive,
    Ausbit,
    Anyx,
}

impl HiveNodes {
    pub fn to_hive_node(self) -> HiveNode {
        use HiveNodes::*;

        let node_str = match self {
            HiveBlog => "https://api.hive.blog",
            Deathwing => "https://api.deathwing.me",
            Arcange => "https://hive-api.arcange.eu",
            Emrebeyler => "https://hived.emre.sh",
            OpenHive => "https://api.openhive.network",
            Ausbit => "https://rpc.ausbit.dev",
            Anyx => "https://anyx.io",
        };

        HiveNode::from(node_str)
    }
}

impl<T: ToString> From<T> for HiveNode {
    fn from(node: T) -> Self {
        Self(node.to_string())
    }
}

impl From<HiveNodes> for HiveNode {
    fn from(hive_nodes: HiveNodes) -> Self {
        hive_nodes.to_hive_node()
    }
}

#[derive(Debug)]
pub enum HiveError {
    RequestFail(reqwest::Error),
    ParseFail(reqwest::Error),
    HiveRejected(String),
    HiveEmptyResponse,
}

#[derive(Deserialize, Debug)]
pub struct HiveResponse<T> {
    pub result: Option<T>,
    pub error: Option<Value>,
    pub id: u64,
}

pub trait ClientMode {}

pub struct MethodMode;
pub struct OperationMode;

impl ClientMode for MethodMode {}
impl ClientMode for OperationMode {}

pub struct HiveClient {
    request_client: Client,
    hive_node: HiveNode,
}

impl HiveClient {
    pub fn new<N: Into<HiveNode>>(hive_node: N) -> Self {
        Self {
            request_client: Client::new(),
            hive_node: hive_node.into(),
        }
    }
    pub fn method(&self) -> HiveClientRef<MethodMode> {
        HiveClientRef {
            hive_client: self,
            id: None,
            client_mode: Default::default(),
        }
    }
    pub fn operation(&self) -> HiveClientRef<OperationMode> {
        HiveClientRef {
            hive_client: self,
            id: None,
            client_mode: Default::default(),
        }
    }
}

pub struct HiveClientRef<'a, M: ClientMode> {
    hive_client: &'a HiveClient,
    id: Option<u64>,
    client_mode: std::marker::PhantomData<M>,
}

impl<'a, M: ClientMode> HiveClientRef<'a, M> {
    pub fn set_id(self, id: u64) -> HiveClientRef<'a, M> {
        HiveClientRef {
            hive_client: self.hive_client,
            id: Some(id),
            client_mode: Default::default(),
        }
    }
}

impl HiveClientRef<'_, MethodMode> {
    pub async fn send<M: Method>(
        self,
        method: M,
    ) -> Result<M::Result, HiveError> {
        self.send_custom_response::<M::Result, M>(method).await
    }
    pub async fn send_custom_response<R: DeserializeOwned, M: Method>(
        self,
        method: M,
    ) -> Result<R, HiveError> {
        use HiveError::*;
        let id = self.id.unwrap_or_else(|| Utc::now().timestamp() as u64);

        self.hive_client.request_client
            .post(&self.hive_client.hive_node.0)
            .body(method.build(id))
            .send()
            .await
            .map_err(RequestFail)?
            .json::<HiveResponse<R>>()
            .await
            .map_err(ParseFail)
            .and_then(|response| response.result.ok_or_else(|| match response.error {
                Some(err) => HiveRejected(err.to_string()),
                None => HiveEmptyResponse,
            }))
    }
}