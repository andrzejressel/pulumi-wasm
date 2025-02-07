#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DbInstanceLogDeliveryConfiguration {
    /// Configuration for S3 bucket log delivery.
    #[builder(into, default)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<Option<super::super::types::timestreaminfluxdb::DbInstanceLogDeliveryConfigurationS3Configuration>>,
}
