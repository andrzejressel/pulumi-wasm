#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SigningProfileSigningMaterial {
    /// The Amazon Resource Name (ARN) of the certificates that is used to sign your code.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: Box<String>,
}