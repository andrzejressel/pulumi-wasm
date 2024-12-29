#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OutboundConnectionLocalDomainInfo {
    /// The name of the local domain.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// The Account ID of the owner of the local domain.
    #[builder(into)]
    #[serde(rename = "ownerId")]
    pub r#owner_id: Box<String>,
    /// The region of the local domain.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
