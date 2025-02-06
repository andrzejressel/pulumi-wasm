#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeRestoreParameters {
    /// Full name of the snapshot to use for creating this volume.
    /// `source_snapshot` and `source_backup` cannot be used simultaneously.
    /// Format: `projects/{{project}}/locations/{{location}}/backupVaults/{{backupVaultId}}/backups/{{backup}}`.
    #[builder(into, default)]
    #[serde(rename = "sourceBackup")]
    pub r#source_backup: Box<Option<String>>,
    /// Full name of the snapshot to use for creating this volume.
    /// `source_snapshot` and `source_backup` cannot be used simultaneously.
    /// Format: `projects/{{project}}/locations/{{location}}/volumes/{{volume}}/snapshots/{{snapshot}}`.
    #[builder(into, default)]
    #[serde(rename = "sourceSnapshot")]
    pub r#source_snapshot: Box<Option<String>>,
}
