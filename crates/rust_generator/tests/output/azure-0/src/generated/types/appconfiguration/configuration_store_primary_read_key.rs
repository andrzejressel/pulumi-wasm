#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationStorePrimaryReadKey {
    /// The Connection String for this Access Key - comprising of the Endpoint, ID and Secret.
    #[builder(into, default)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<Option<String>>,
    /// The ID of the Access Key.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Secret of the Access Key.
    #[builder(into, default)]
    #[serde(rename = "secret")]
    pub r#secret: Box<Option<String>>,
}
