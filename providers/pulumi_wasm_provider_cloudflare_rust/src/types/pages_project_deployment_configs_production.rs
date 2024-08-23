#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProduction {
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Box<Option<bool>>,
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Box<Option<String>>,
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Box<Option<Vec<String>>>,
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::PagesProjectDeploymentConfigsProductionPlacement>>,
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings:
        Box<Option<Vec<crate::types::PagesProjectDeploymentConfigsProductionServiceBinding>>>,
    #[serde(rename = "usageModel")]
    pub r#usage_model: Box<Option<String>>,
}
