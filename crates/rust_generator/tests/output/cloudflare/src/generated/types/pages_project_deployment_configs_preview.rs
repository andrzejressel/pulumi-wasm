#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PagesProjectDeploymentConfigsPreview {
    /// Use latest compatibility date for Pages Functions. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Box<Option<bool>>,
    /// Compatibility date used for Pages Functions.
    #[builder(into, default)]
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Box<Option<String>>,
    /// Compatibility flags used for Pages Functions.
    #[builder(into, default)]
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Box<Option<Vec<String>>>,
    /// D1 Databases used for Pages Functions. Defaults to `map[]`.
    #[builder(into, default)]
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Box<Option<std::collections::HashMap<String, String>>>,
    /// Durable Object namespaces used for Pages Functions. Defaults to `map[]`.
    #[builder(into, default)]
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    /// Environment variables for Pages Functions. Defaults to `map[]`.
    #[builder(into, default)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// Fail open used for Pages Functions. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    /// KV namespaces used for Pages Functions. Defaults to `map[]`.
    #[builder(into, default)]
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    /// Configuration for placement in the Cloudflare Pages project.
    #[builder(into, default)]
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<super::types::PagesProjectDeploymentConfigsPreviewPlacement>>,
    /// R2 Buckets used for Pages Functions. Defaults to `map[]`.
    #[builder(into, default)]
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Encrypted environment variables for Pages Functions. Defaults to `map[]`.
    #[builder(into, default)]
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Services used for Pages Functions.
    #[builder(into, default)]
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings: Box<Option<Vec<super::types::PagesProjectDeploymentConfigsPreviewServiceBinding>>>,
    /// Usage model used for Pages Functions. Available values: `unbound`, `bundled`, `standard`. Defaults to `bundled`.
    #[builder(into, default)]
    #[serde(rename = "usageModel")]
    pub r#usage_model: Box<Option<String>>,
}
