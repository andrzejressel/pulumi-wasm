#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreview {
    /// Use latest compatibility date for Pages Functions. Defaults to `false`.
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Box<Option<bool>>,
    /// Compatibility date used for Pages Functions.
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Box<Option<String>>,
    /// Compatibility flags used for Pages Functions.
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Box<Option<Vec<String>>>,
    /// D1 Databases used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Box<Option<std::collections::HashMap<String, String>>>,
    /// Durable Object namespaces used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    /// Environment variables for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// Fail open used for Pages Functions. Defaults to `false`.
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    /// KV namespaces used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    /// Configuration for placement in the Cloudflare Pages project.
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::PagesProjectDeploymentConfigsPreviewPlacement>>,
    /// R2 Buckets used for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Encrypted environment variables for Pages Functions. Defaults to `map[]`.
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Services used for Pages Functions.
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings:
        Box<Option<Vec<crate::types::PagesProjectDeploymentConfigsPreviewServiceBinding>>>,
    /// Usage model used for Pages Functions. Available values: `unbound`, `bundled`, `standard`. Defaults to `bundled`.
    #[serde(rename = "usageModel")]
    pub r#usage_model: Box<Option<String>>,
}
