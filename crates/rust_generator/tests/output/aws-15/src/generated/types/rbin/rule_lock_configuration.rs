#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleLockConfiguration {
    /// Information about the retention rule unlock delay. See `unlock_delay` below.
    #[builder(into)]
    #[serde(rename = "unlockDelay")]
    pub r#unlock_delay: Box<super::super::types::rbin::RuleLockConfigurationUnlockDelay>,
}
