#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateSpecVolume {
    /// A filesystem specified by the Container Storage Interface (CSI).
    #[builder(into)]
    #[serde(rename = "csis")]
    pub r#csis: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecVolumeCsi>>,
    /// Ephemeral storage which can be backed by real disks (HD, SSD), network storage or memory (i.e. tmpfs). For now only in memory (tmpfs) is supported. It is ephemeral in the sense that when the sandbox is taken down, the data is destroyed with it (it does not persist across sandbox runs).
    #[builder(into)]
    #[serde(rename = "emptyDirs")]
    pub r#empty_dirs: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecVolumeEmptyDir>>,
    /// The name of the Cloud Run Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A filesystem backed by a Network File System share. This filesystem requires the
    /// run.googleapis.com/execution-environment annotation to be unset or set to "gen2"
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecVolumeNf>>,
    /// The secret's value will be presented as the content of a file whose
    /// name is defined in the item path. If no items are defined, the name of
    /// the file is the secret_name.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecVolumeSecret>>,
}
