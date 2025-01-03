#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RestorePlanRestoreConfigClusterResourceRestoreScopeSelectedGroupKind {
    /// API Group string of a Kubernetes resource, e.g.
    /// "apiextensions.k8s.io", "storage.k8s.io", etc.
    /// Use empty string for core group.
    #[builder(into, default)]
    #[serde(rename = "resourceGroup")]
    pub r#resource_group: Box<Option<String>>,
    /// Kind of a Kubernetes resource, e.g.
    /// "CustomResourceDefinition", "StorageClass", etc.
    #[builder(into, default)]
    #[serde(rename = "resourceKind")]
    pub r#resource_kind: Box<Option<String>>,
}
