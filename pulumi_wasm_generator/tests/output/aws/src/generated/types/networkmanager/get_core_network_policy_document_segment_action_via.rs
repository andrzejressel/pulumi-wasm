#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCoreNetworkPolicyDocumentSegmentActionVia {
    /// A list of strings. The network function group to use for the service insertion action.
    #[builder(into, default)]
    #[serde(rename = "networkFunctionGroups")]
    pub r#network_function_groups: Box<Option<Vec<String>>>,
    /// Any edge overrides and the preferred edge to use.
    #[builder(into, default)]
    #[serde(rename = "withEdgeOverrides")]
    pub r#with_edge_overrides: Box<Option<Vec<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentActionViaWithEdgeOverride>>>,
}
