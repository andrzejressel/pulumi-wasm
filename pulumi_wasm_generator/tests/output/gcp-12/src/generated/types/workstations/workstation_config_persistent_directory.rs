#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkstationConfigPersistentDirectory {
    /// A directory to persist across workstation sessions, backed by a Compute Engine regional persistent disk. Can only be updated if not empty during creation.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gcePd")]
    pub r#gce_pd: Box<Option<super::super::types::workstations::WorkstationConfigPersistentDirectoryGcePd>>,
    /// Location of this directory in the running workstation.
    #[builder(into, default)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<Option<String>>,
}
