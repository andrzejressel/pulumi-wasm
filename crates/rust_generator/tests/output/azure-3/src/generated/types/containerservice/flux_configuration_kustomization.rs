#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FluxConfigurationKustomization {
    /// Specifies other kustomizations that this kustomization depends on. This kustomization will not reconcile until all dependencies have completed their reconciliation.
    #[builder(into, default)]
    #[serde(rename = "dependsOns")]
    pub r#depends_ons: Box<Option<Vec<String>>>,
    /// Whether garbage collections of Kubernetes objects created by this kustomization is enabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "garbageCollectionEnabled")]
    pub r#garbage_collection_enabled: Box<Option<bool>>,
    /// Specifies the name of the kustomization.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the path in the source reference to reconcile on the cluster.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Whether re-creating Kubernetes resources on the cluster is enabled when patching fails due to an immutable field change. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "recreatingEnabled")]
    pub r#recreating_enabled: Box<Option<bool>>,
    /// The interval at which to re-reconcile the kustomization on the cluster in the event of failure on reconciliation. Defaults to `600`.
    #[builder(into, default)]
    #[serde(rename = "retryIntervalInSeconds")]
    pub r#retry_interval_in_seconds: Box<Option<i32>>,
    /// The interval at which to re-reconcile the kustomization on the cluster. Defaults to `600`.
    #[builder(into, default)]
    #[serde(rename = "syncIntervalInSeconds")]
    pub r#sync_interval_in_seconds: Box<Option<i32>>,
    /// The maximum time to attempt to reconcile the kustomization on the cluster. Defaults to `600`.
    #[builder(into, default)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<Option<i32>>,
}
