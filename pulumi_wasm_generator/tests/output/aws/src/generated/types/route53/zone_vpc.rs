#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZoneVpc {
    /// ID of the VPC to associate.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
    /// Region of the VPC to associate. Defaults to AWS provider region.
    #[builder(into, default)]
    #[serde(rename = "vpcRegion")]
    pub r#vpc_region: Box<Option<String>>,
}