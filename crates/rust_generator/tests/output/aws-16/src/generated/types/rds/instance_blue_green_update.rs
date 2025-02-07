#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceBlueGreenUpdate {
    /// Enables low-downtime updates when `true`.
    /// Default is `false`.
    /// 
    /// [instance-replication]:
    /// https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Overview.Replication.html
    /// [instance-maintenance]:
    /// https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Maintenance.html
    /// [blue-green]:
    /// https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/blue-green-deployments.html
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
