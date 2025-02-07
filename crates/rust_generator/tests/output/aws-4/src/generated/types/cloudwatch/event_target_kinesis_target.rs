#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventTargetKinesisTarget {
    /// The JSON path to be extracted from the event and used as the partition key.
    #[builder(into, default)]
    #[serde(rename = "partitionKeyPath")]
    pub r#partition_key_path: Box<Option<String>>,
}
