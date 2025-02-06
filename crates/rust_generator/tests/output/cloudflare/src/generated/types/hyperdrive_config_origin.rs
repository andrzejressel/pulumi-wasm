#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HyperdriveConfigOrigin {
    /// Client ID associated with the Cloudflare Access Service Token used to connect via Access.
    #[builder(into, default)]
    #[serde(rename = "accessClientId")]
    pub r#access_client_id: Box<Option<String>>,
    /// Client Secret associated with the Cloudflare Access Service Token used to connect via Access.
    #[builder(into, default)]
    #[serde(rename = "accessClientSecret")]
    pub r#access_client_secret: Box<Option<String>>,
    /// The name of your origin database.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    /// The host (hostname or IP) of your origin database.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The password of the Hyperdrive configuration.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The port (default: 5432 for Postgres) of your origin database.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Specifies the URL scheme used to connect to your origin database.
    #[builder(into)]
    #[serde(rename = "scheme")]
    pub r#scheme: Box<String>,
    /// The user of your origin database.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Box<String>,
}
