#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StorageLensConfigurationStorageLensConfigurationAwsOrg {
    /// The Amazon Resource Name (ARN) of the Amazon Web Services organization.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
}