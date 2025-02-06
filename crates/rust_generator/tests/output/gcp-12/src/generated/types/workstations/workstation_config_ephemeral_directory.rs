#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkstationConfigEphemeralDirectory {
    /// An EphemeralDirectory backed by a Compute Engine persistent disk.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gcePd")]
    pub r#gce_pd: Box<Option<super::super::types::workstations::WorkstationConfigEphemeralDirectoryGcePd>>,
    /// Location of this directory in the running workstation.
    #[builder(into, default)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<Option<String>>,
}
