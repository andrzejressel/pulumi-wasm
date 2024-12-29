#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBrokerEngineTypesBrokerEngineType {
    /// The MQ engine type to return version details for.
    #[builder(into)]
    #[serde(rename = "engineType")]
    pub r#engine_type: Box<String>,
    /// The list of engine versions.
    #[builder(into)]
    #[serde(rename = "engineVersions")]
    pub r#engine_versions: Box<Vec<super::super::types::mq::GetBrokerEngineTypesBrokerEngineTypeEngineVersion>>,
}
