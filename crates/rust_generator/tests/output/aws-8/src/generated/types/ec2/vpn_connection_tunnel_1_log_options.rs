#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnConnectionTunnel1LogOptions {
    /// Options for sending VPN tunnel logs to CloudWatch. See CloudWatch Log Options below for more details.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogOptions")]
    pub r#cloudwatch_log_options: Box<Option<super::super::types::ec2::VpnConnectionTunnel1LogOptionsCloudwatchLogOptions>>,
}
