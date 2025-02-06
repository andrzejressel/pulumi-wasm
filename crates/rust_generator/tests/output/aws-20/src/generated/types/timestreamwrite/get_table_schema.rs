#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTableSchema {
    #[builder(into)]
    #[serde(rename = "compositePartitionKeys")]
    pub r#composite_partition_keys: Box<Vec<super::super::types::timestreamwrite::GetTableSchemaCompositePartitionKey>>,
}
