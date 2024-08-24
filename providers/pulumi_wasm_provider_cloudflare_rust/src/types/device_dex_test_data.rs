#[derive(serde::Serialize)]
pub struct DeviceDexTestData {
    /// The host URL for `http` test `kind`. For `traceroute`, it must be a valid hostname or IP address.
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The type of Device Dex Test. Available values: `http`, `traceroute`.
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    /// The http request method. Available values: `GET`.
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
}
