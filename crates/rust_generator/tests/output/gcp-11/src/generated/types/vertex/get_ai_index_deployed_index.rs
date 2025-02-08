#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAiIndexDeployedIndex {
    /// The ID of the DeployedIndex in the above IndexEndpoint.
    #[builder(into)]
    #[serde(rename = "deployedIndexId")]
    pub r#deployed_index_id: Box<String>,
    /// A resource name of the IndexEndpoint.
    #[builder(into)]
    #[serde(rename = "indexEndpoint")]
    pub r#index_endpoint: Box<String>,
}
