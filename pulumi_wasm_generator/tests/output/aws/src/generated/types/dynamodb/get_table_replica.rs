#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTableReplica {
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "regionName")]
    pub r#region_name: Box<String>,
}