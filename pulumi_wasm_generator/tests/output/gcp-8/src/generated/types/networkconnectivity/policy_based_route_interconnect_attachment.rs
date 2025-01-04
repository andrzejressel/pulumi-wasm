#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyBasedRouteInterconnectAttachment {
    /// Cloud region to install this policy-based route on for Interconnect attachments. Use `all` to install it on all Interconnect attachments.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
