#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryWorkflowConfigInvocationConfig {
    /// Optional. When set to true, any incremental tables will be fully refreshed.
    #[builder(into, default)]
    #[serde(rename = "fullyRefreshIncrementalTablesEnabled")]
    pub r#fully_refresh_incremental_tables_enabled: Box<Option<bool>>,
    /// Optional. The set of tags to include.
    #[builder(into, default)]
    #[serde(rename = "includedTags")]
    pub r#included_tags: Box<Option<Vec<String>>>,
    /// Optional. The set of action identifiers to include.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "includedTargets")]
    pub r#included_targets: Box<Option<Vec<super::super::types::dataform::RepositoryWorkflowConfigInvocationConfigIncludedTarget>>>,
    /// Optional. The service account to run workflow invocations under.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
    /// Optional. When set to true, transitive dependencies of included actions will be executed.
    #[builder(into, default)]
    #[serde(rename = "transitiveDependenciesIncluded")]
    pub r#transitive_dependencies_included: Box<Option<bool>>,
    /// Optional. When set to true, transitive dependents of included actions will be executed.
    #[builder(into, default)]
    #[serde(rename = "transitiveDependentsIncluded")]
    pub r#transitive_dependents_included: Box<Option<bool>>,
}
