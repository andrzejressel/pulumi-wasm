#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateVpcAccess {
    /// VPC Access connector name. Format: projects/{project}/locations/{location}/connectors/{connector}, where {project} can be project id or number.
    #[builder(into, default)]
    #[serde(rename = "connector")]
    pub r#connector: Box<Option<String>>,
    /// Traffic VPC egress settings.
    /// Possible values are: `ALL_TRAFFIC`, `PRIVATE_RANGES_ONLY`.
    #[builder(into, default)]
    #[serde(rename = "egress")]
    pub r#egress: Box<Option<String>>,
    /// Direct VPC egress settings. Currently only single network interface is supported.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Box<Option<Vec<super::super::types::cloudrunv2::ServiceTemplateVpcAccessNetworkInterface>>>,
}
