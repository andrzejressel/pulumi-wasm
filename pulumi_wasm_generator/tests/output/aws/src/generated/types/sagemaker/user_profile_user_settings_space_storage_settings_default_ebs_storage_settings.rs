#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserProfileUserSettingsSpaceStorageSettingsDefaultEbsStorageSettings {
    /// The default size of the EBS storage volume for a private space.
    #[builder(into)]
    #[serde(rename = "defaultEbsVolumeSizeInGb")]
    pub r#default_ebs_volume_size_in_gb: Box<i32>,
    /// The maximum size of the EBS storage volume for a private space.
    #[builder(into)]
    #[serde(rename = "maximumEbsVolumeSizeInGb")]
    pub r#maximum_ebs_volume_size_in_gb: Box<i32>,
}
