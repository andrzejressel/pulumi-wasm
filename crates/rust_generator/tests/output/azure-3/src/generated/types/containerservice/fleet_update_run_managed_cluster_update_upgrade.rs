#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetUpdateRunManagedClusterUpdateUpgrade {
    /// Specifies the Kubernetes version to upgrade the member clusters to. This is required if `type` is set to `Full`.
    #[builder(into, default)]
    #[serde(rename = "kubernetesVersion")]
    pub r#kubernetes_version: Box<Option<String>>,
    /// Specifies the type of upgrade to perform. Possible values are `Full` and `NodeImageOnly`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
