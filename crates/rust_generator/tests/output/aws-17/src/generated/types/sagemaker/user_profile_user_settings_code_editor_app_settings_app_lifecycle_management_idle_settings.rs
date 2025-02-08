#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserProfileUserSettingsCodeEditorAppSettingsAppLifecycleManagementIdleSettings {
    /// The time that SageMaker waits after the application becomes idle before shutting it down. Valid values are between `60` and `525600`.
    #[builder(into, default)]
    #[serde(rename = "idleTimeoutInMinutes")]
    pub r#idle_timeout_in_minutes: Box<Option<i32>>,
    /// Indicates whether idle shutdown is activated for the application type. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "lifecycleManagement")]
    pub r#lifecycle_management: Box<Option<String>>,
    /// The maximum value in minutes that custom idle shutdown can be set to by the user. Valid values are between `60` and `525600`.
    #[builder(into, default)]
    #[serde(rename = "maxIdleTimeoutInMinutes")]
    pub r#max_idle_timeout_in_minutes: Box<Option<i32>>,
    /// The minimum value in minutes that custom idle shutdown can be set to by the user. Valid values are between `60` and `525600`.
    #[builder(into, default)]
    #[serde(rename = "minIdleTimeoutInMinutes")]
    pub r#min_idle_timeout_in_minutes: Box<Option<i32>>,
}
