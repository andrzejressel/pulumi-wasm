#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTableEntitiesItem {
    /// Partition Key of the Entity.
    #[builder(into)]
    #[serde(rename = "partitionKey")]
    pub r#partition_key: Box<String>,
    /// A map of any additional properties in key-value format.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Box<std::collections::HashMap<String, String>>,
    /// Row Key of the Entity.
    #[builder(into)]
    #[serde(rename = "rowKey")]
    pub r#row_key: Box<String>,
}
