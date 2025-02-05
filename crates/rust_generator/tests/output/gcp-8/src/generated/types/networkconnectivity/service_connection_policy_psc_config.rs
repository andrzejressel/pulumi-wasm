#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceConnectionPolicyPscConfig {
    /// Max number of PSC connections for this policy.
    #[builder(into, default)]
    #[serde(rename = "limit")]
    pub r#limit: Box<Option<String>>,
    /// IDs of the subnetworks or fully qualified identifiers for the subnetworks
    #[builder(into)]
    #[serde(rename = "subnetworks")]
    pub r#subnetworks: Box<Vec<String>>,
}
