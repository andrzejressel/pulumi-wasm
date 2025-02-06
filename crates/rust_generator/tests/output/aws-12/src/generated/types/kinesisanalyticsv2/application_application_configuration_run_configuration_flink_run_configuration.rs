#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationRunConfigurationFlinkRunConfiguration {
    /// When restoring from a snapshot, specifies whether the runtime is allowed to skip a state that cannot be mapped to the new program. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "allowNonRestoredState")]
    pub r#allow_non_restored_state: Box<Option<bool>>,
}
