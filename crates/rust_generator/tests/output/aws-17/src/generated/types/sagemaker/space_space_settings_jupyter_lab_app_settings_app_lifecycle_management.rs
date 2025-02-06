#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpaceSpaceSettingsJupyterLabAppSettingsAppLifecycleManagement {
    /// Settings related to idle shutdown of Studio applications. See `idle_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "idleSettings")]
    pub r#idle_settings: Box<Option<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsAppLifecycleManagementIdleSettings>>,
}
