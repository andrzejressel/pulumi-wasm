#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPrivateCloudManagementCluster {
    /// The list of the hosts in the management cluster.
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Vec<String>>,
    /// The ID of the management cluster.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<i32>,
    /// The size of the management cluster.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
}
