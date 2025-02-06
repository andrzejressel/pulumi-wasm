#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchTemplateMaintenanceOptions {
    /// Disables the automatic recovery behavior of your instance or sets it to default. Can be `"default"` or `"disabled"`. See [Recover your instance](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-recover.html) for more details.
    #[builder(into, default)]
    #[serde(rename = "autoRecovery")]
    pub r#auto_recovery: Box<Option<String>>,
}
