#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Registry {
    /// The password to authenticate to the registry. Does not cause image rebuild when changed.
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The URL of the Docker registry server
    #[serde(rename = "server")]
    pub r#server: Box<Option<String>>,
    /// The username to authenticate to the registry. Does not cause image rebuild when changed.
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
