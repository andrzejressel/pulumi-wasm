#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionOriginVpcOriginConfig {
    #[builder(into, default)]
    #[serde(rename = "originKeepaliveTimeout")]
    pub r#origin_keepalive_timeout: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "originReadTimeout")]
    pub r#origin_read_timeout: Box<Option<i32>>,
    /// The VPC origin ID.
    #[builder(into)]
    #[serde(rename = "vpcOriginId")]
    pub r#vpc_origin_id: Box<String>,
}
