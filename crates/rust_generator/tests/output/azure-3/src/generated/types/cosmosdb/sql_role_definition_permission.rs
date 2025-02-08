#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SqlRoleDefinitionPermission {
    /// A list of data actions that are allowed for the Cosmos DB SQL Role Definition.
    #[builder(into)]
    #[serde(rename = "dataActions")]
    pub r#data_actions: Box<Vec<String>>,
}
