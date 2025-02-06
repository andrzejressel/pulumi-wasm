#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNetworkDdosProtectionPlan {
    /// Enable/disable DDoS Protection Plan on Virtual Network.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Box<bool>,
    /// The ID of DDoS Protection Plan.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
