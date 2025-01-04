#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterApiServerProfile {
    /// The IP Address the Ingress Profile is associated with.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// The URL the API Server Profile is associated with.
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
    /// Cluster API server visibility. Supported values are `Public` and `Private`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "visibility")]
    pub r#visibility: Box<String>,
}
