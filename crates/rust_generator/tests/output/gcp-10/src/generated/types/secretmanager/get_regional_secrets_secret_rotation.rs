#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionalSecretsSecretRotation {
    /// Timestamp in UTC at which the secret is scheduled to rotate.
    #[builder(into)]
    #[serde(rename = "nextRotationTime")]
    pub r#next_rotation_time: Box<String>,
    /// The Duration between rotation notifications.
    #[builder(into)]
    #[serde(rename = "rotationPeriod")]
    pub r#rotation_period: Box<String>,
}
