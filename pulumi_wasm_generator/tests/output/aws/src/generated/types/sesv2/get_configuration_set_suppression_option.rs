#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationSetSuppressionOption {
    /// A list that contains the reasons that email addresses are automatically added to the suppression list for your account.
    #[builder(into)]
    #[serde(rename = "suppressedReasons")]
    pub r#suppressed_reasons: Box<Vec<String>>,
}