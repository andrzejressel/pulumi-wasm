#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImageOutputResourceAmi {
    /// Account identifier of the AMI.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<String>,
    /// Description of the AMI.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Identifier of the AMI.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// Name of the AMI.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Region of the container image.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
