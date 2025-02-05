#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterOutpostConfigControlPlanePlacement {
    /// The name of the placement group for the Kubernetes control plane instances.
    #[builder(into)]
    #[serde(rename = "groupName")]
    pub r#group_name: Box<String>,
}
