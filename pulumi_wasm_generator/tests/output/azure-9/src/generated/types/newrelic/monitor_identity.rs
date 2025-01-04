#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MonitorIdentity {
    /// The Principal ID for the Service Principal associated with the Identity of this Azure Native New Relic Monitor.
    #[builder(into, default)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<Option<String>>,
    /// The Tenant ID for the Service Principal associated with the Identity of this Azure Native New Relic Monitor.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
    /// Specifies the identity type of the Azure Native New Relic Monitor. The only possible value is `SystemAssigned`. Changing this forces a new Azure Native New Relic Monitor to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
