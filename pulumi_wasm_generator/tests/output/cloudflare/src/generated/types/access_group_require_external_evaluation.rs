#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessGroupRequireExternalEvaluation {
    /// The API endpoint containing your business logic.
    #[builder(into, default)]
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    /// The API endpoint containing the key that Access uses to verify that the response came from your API.
    #[builder(into, default)]
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}
