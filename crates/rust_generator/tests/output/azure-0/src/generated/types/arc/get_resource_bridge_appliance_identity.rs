#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResourceBridgeApplianceIdentity {
    /// The Principal ID associated with this Managed Service Identity.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Tenant ID associated with this Managed Service Identity.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The type of this Managed Service Identity.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
