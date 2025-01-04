#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBackendServiceLocalityLbPolicyCustomPolicy {
    /// An optional, arbitrary JSON object with configuration data, understood
    /// by a locally installed custom policy implementation.
    #[builder(into)]
    #[serde(rename = "data")]
    pub r#data: Box<String>,
    /// The name of the Backend Service.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
