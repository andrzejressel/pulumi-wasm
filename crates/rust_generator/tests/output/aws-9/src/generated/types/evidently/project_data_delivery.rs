#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProjectDataDelivery {
    /// A block that defines the CloudWatch Log Group that stores the evaluation events. See below.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Box<Option<super::super::types::evidently::ProjectDataDeliveryCloudwatchLogs>>,
    /// A block that defines the S3 bucket and prefix that stores the evaluation events. See below.
    #[builder(into, default)]
    #[serde(rename = "s3Destination")]
    pub r#s_3_destination: Box<Option<super::super::types::evidently::ProjectDataDeliveryS3Destination>>,
}
