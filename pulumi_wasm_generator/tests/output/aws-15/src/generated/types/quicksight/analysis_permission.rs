#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalysisPermission {
    /// List of IAM actions to grant or revoke permissions on.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<String>>,
    /// ARN of the principal. See the [ResourcePermission documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ResourcePermission.html) for the applicable ARN values.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Box<String>,
}
