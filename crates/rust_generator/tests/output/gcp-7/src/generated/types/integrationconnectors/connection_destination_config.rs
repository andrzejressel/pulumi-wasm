#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionDestinationConfig {
    /// The destinations for the key.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Option<Vec<super::super::types::integrationconnectors::ConnectionDestinationConfigDestination>>>,
    /// The key is the destination identifier that is supported by the Connector.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}
