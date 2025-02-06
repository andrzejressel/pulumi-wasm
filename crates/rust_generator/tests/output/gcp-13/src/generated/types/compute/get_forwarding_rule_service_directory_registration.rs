#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetForwardingRuleServiceDirectoryRegistration {
    /// Service Directory namespace to register the forwarding rule under.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<String>,
    /// Service Directory service to register the forwarding rule under.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
