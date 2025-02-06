#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicRuleTimestreamDimension {
    /// The metadata dimension name. This is the name of the column in the Amazon Timestream database table record.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value to write in this column of the database record.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
