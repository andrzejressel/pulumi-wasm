#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableGlobalSecondaryIndex {
    /// Name of the hash key in the index; must be defined as an attribute in the resource.
    #[builder(into)]
    #[serde(rename = "hashKey")]
    pub r#hash_key: Box<String>,
    /// Name of the index.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Only required with `INCLUDE` as a projection type; a list of attributes to project into the index. These do not need to be defined as attributes on the table.
    #[builder(into, default)]
    #[serde(rename = "nonKeyAttributes")]
    pub r#non_key_attributes: Box<Option<Vec<String>>>,
    /// Sets the maximum number of read and write units for the specified on-demand table. See below.
    #[builder(into, default)]
    #[serde(rename = "onDemandThroughput")]
    pub r#on_demand_throughput: Box<Option<super::super::types::dynamodb::TableGlobalSecondaryIndexOnDemandThroughput>>,
    /// One of `ALL`, `INCLUDE` or `KEYS_ONLY` where `ALL` projects every attribute into the index, `KEYS_ONLY` projects  into the index only the table and index hash_key and sort_key attributes ,  `INCLUDE` projects into the index all of the attributes that are defined in `non_key_attributes` in addition to the attributes that that`KEYS_ONLY` project.
    #[builder(into)]
    #[serde(rename = "projectionType")]
    pub r#projection_type: Box<String>,
    /// Name of the range key; must be defined
    #[builder(into, default)]
    #[serde(rename = "rangeKey")]
    pub r#range_key: Box<Option<String>>,
    /// Number of read units for this index. Must be set if billing_mode is set to PROVISIONED.
    #[builder(into, default)]
    #[serde(rename = "readCapacity")]
    pub r#read_capacity: Box<Option<i32>>,
    /// Number of write units for this index. Must be set if billing_mode is set to PROVISIONED.
    #[builder(into, default)]
    #[serde(rename = "writeCapacity")]
    pub r#write_capacity: Box<Option<i32>>,
}
