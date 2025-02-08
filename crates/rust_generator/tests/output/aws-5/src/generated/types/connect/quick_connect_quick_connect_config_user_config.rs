#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct QuickConnectQuickConnectConfigUserConfig {
    /// Specifies the identifier of the contact flow.
    #[builder(into)]
    #[serde(rename = "contactFlowId")]
    pub r#contact_flow_id: Box<String>,
    /// Specifies the identifier for the user.
    #[builder(into)]
    #[serde(rename = "userId")]
    pub r#user_id: Box<String>,
}
