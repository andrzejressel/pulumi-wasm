#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceStorageConfigStorageConfigKinesisFirehoseConfig {
    /// The Amazon Resource Name (ARN) of the delivery stream.
    #[builder(into)]
    #[serde(rename = "firehoseArn")]
    pub r#firehose_arn: Box<String>,
}
