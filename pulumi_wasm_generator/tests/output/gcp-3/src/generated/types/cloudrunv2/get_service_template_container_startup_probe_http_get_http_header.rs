#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateContainerStartupProbeHttpGetHttpHeader {
    /// The name of the Cloud Run v2 Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The header field value
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
