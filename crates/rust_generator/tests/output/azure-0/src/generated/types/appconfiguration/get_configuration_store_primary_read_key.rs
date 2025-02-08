#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetConfigurationStorePrimaryReadKey {
    /// The Connection String for this Access Key - comprising of the Endpoint, ID and Secret.
    #[builder(into)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<String>,
    /// The ID of the Access Key.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The Secret of the Access Key.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Box<String>,
}
