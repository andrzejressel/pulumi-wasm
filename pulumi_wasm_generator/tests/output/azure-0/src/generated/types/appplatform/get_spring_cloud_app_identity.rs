#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSpringCloudAppIdentity {
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The Principal ID for the Service Principal associated with the Managed Service Identity of this Spring Cloud Application.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Tenant ID for the Service Principal associated with the Managed Service Identity of this Spring Cloud Application.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The Type of Managed Identity assigned to the Spring Cloud Application.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
