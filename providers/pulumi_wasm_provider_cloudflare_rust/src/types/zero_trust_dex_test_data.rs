#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDexTestData {
    /// The host URL for `http` test `kind`. For `traceroute`, it must be a valid hostname or IP address.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The type of Device Dex Test. Available values: `http`, `traceroute`.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    /// The http request method. Available values: `GET`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
}
