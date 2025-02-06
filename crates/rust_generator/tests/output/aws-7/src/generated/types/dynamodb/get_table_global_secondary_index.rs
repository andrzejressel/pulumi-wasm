#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTableGlobalSecondaryIndex {
    #[builder(into)]
    #[serde(rename = "hashKey")]
    pub r#hash_key: Box<String>,
    /// Name of the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "nonKeyAttributes")]
    pub r#non_key_attributes: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "projectionType")]
    pub r#projection_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "rangeKey")]
    pub r#range_key: Box<String>,
    #[builder(into)]
    #[serde(rename = "readCapacity")]
    pub r#read_capacity: Box<i32>,
    #[builder(into)]
    #[serde(rename = "writeCapacity")]
    pub r#write_capacity: Box<i32>,
}
