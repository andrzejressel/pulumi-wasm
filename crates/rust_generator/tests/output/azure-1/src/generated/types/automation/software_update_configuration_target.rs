#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SoftwareUpdateConfigurationTarget {
    /// One or more `azure_query` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "azureQueries")]
    pub r#azure_queries: Box<Option<Vec<super::super::types::automation::SoftwareUpdateConfigurationTargetAzureQuery>>>,
    /// One or more `non_azure_query` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "nonAzureQueries")]
    pub r#non_azure_queries: Box<Option<Vec<super::super::types::automation::SoftwareUpdateConfigurationTargetNonAzureQuery>>>,
}
