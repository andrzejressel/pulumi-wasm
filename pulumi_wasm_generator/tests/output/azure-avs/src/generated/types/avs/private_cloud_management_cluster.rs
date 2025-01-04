#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PrivateCloudManagementCluster {
    /// A list of hosts in the management cluster.
    #[builder(into, default)]
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Option<Vec<String>>>,
    /// The ID of the management cluster.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<i32>>,
    /// The size of the management cluster. This field can not updated with `internet_connection_enabled` together.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
}
