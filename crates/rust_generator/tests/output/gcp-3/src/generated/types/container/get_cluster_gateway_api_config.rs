#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterGatewayApiConfig {
    /// The Gateway API release channel to use for Gateway API.
    #[builder(into)]
    #[serde(rename = "channel")]
    pub r#channel: Box<String>,
}
