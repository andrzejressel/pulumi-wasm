#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UsagePlanApiStage {
    /// API Id of the associated API stage in a usage plan.
    #[builder(into)]
    #[serde(rename = "apiId")]
    pub r#api_id: Box<String>,
    /// API stage name of the associated API stage in a usage plan.
    #[builder(into)]
    #[serde(rename = "stage")]
    pub r#stage: Box<String>,
    /// The throttling limits of the usage plan.
    #[builder(into, default)]
    #[serde(rename = "throttles")]
    pub r#throttles: Box<Option<Vec<super::super::types::apigateway::UsagePlanApiStageThrottle>>>,
}
