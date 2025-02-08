#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VpnConnectionTunnel2LogOptionsCloudwatchLogOptions {
    /// Enable or disable VPN tunnel logging feature. The default is `false`.
    #[builder(into, default)]
    #[serde(rename = "logEnabled")]
    pub r#log_enabled: Box<Option<bool>>,
    /// The Amazon Resource Name (ARN) of the CloudWatch log group to send logs to.
    #[builder(into, default)]
    #[serde(rename = "logGroupArn")]
    pub r#log_group_arn: Box<Option<String>>,
    /// Set log format. Default format is json. Possible values are: `json` and `text`. The default is `json`.
    #[builder(into, default)]
    #[serde(rename = "logOutputFormat")]
    pub r#log_output_format: Box<Option<String>>,
}
