#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserProfileUserSettingsSpaceStorageSettings {
    /// The default EBS storage settings for a private space. See Default EBS Storage Settings below.
    #[builder(into, default)]
    #[serde(rename = "defaultEbsStorageSettings")]
    pub r#default_ebs_storage_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsSpaceStorageSettingsDefaultEbsStorageSettings>>,
}
