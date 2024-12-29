#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles {
    /// Unique name of the Amazon Connect Customer Profiles domain.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// Object specified in the Amazon Connect Customer Profiles flow destination.
    #[builder(into, default)]
    #[serde(rename = "objectTypeName")]
    pub r#object_type_name: Box<Option<String>>,
}
