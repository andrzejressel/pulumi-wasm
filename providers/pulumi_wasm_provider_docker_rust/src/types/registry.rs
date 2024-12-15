//! Describes a Docker container registry

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct Registry {
    /// The password to authenticate to the registry. Does not cause image rebuild when changed.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The URL of the Docker registry server
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "server")]
    pub r#server: Box<Option<String>>,
    /// The username to authenticate to the registry. Does not cause image rebuild when changed.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
