#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PermissionsLfTagPolicyExpression {
    /// The key-name of an LF-Tag.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// A list of possible values of an LF-Tag.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
