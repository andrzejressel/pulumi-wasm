#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OutboundRuleFrontendIpConfiguration {
    /// The ID of the Load Balancer Outbound Rule.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The name of the Frontend IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
