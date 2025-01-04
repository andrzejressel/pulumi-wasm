#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourceSetResourceDnsTargetResourceTargetResourceR53Resource {
    /// Domain name that is targeted.
    #[builder(into, default)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<Option<String>>,
    /// Resource record set ID that is targeted.
    #[builder(into, default)]
    #[serde(rename = "recordSetId")]
    pub r#record_set_id: Box<Option<String>>,
}
