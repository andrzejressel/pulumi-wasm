#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicationConfigurationReplicationConfigurationRuleRepositoryFilter {
    /// The repository filter details.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Box<String>,
    /// The repository filter type. The only supported value is `PREFIX_MATCH`, which is a repository name prefix specified with the filter parameter.
    #[builder(into)]
    #[serde(rename = "filterType")]
    pub r#filter_type: Box<String>,
}