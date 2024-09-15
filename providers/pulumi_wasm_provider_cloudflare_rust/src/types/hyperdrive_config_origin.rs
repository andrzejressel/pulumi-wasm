#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct HyperdriveConfigOrigin {
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
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// Specifies the URL scheme used to connect to your origin database.
    #[builder(into)]
    #[serde(rename = "scheme")]
    pub r#scheme: Box<String>,
    /// The user of your origin database.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Box<String>,
}
