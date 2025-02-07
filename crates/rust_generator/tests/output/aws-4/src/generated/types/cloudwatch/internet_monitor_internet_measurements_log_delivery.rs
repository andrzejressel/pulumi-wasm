#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InternetMonitorInternetMeasurementsLogDelivery {
    #[builder(into, default)]
    #[serde(rename = "s3Config")]
    pub r#s_3_config: Box<Option<super::super::types::cloudwatch::InternetMonitorInternetMeasurementsLogDeliveryS3Config>>,
}
