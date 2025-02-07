#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionalSecretRotation {
    /// Timestamp in UTC at which the Secret is scheduled to rotate.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
    /// fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "nextRotationTime")]
    pub r#next_rotation_time: Box<String>,
    /// The Duration between rotation notifications. Must be in seconds and at least 3600s (1h)
    /// and at most 3153600000s (100 years). If rotationPeriod is set, 'next_rotation_time' must
    /// be set. 'next_rotation_time' will be advanced by this period when the service
    /// automatically sends rotation notifications.
    #[builder(into)]
    #[serde(rename = "rotationPeriod")]
    pub r#rotation_period: Box<String>,
}
