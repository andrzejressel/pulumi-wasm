#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AttachedClusterBinaryAuthorization {
    /// Configure Binary Authorization evaluation mode.
    /// Possible values are: `DISABLED`, `PROJECT_SINGLETON_POLICY_ENFORCE`.
    #[builder(into, default)]
    #[serde(rename = "evaluationMode")]
    pub r#evaluation_mode: Box<Option<String>>,
}
