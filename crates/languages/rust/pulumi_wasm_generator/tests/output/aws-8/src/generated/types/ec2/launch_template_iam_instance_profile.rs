#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchTemplateIamInstanceProfile {
    /// The Amazon Resource Name (ARN) of the instance profile. Conflicts with `name`.
    #[builder(into, default)]
    #[serde(rename = "arn")]
    pub r#arn: Box<Option<String>>,
    /// The name of the instance profile.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
