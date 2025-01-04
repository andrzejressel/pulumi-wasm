#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyVmTieringPolicy {
    /// An `archived_restore_point` block as defined below.
    #[builder(into)]
    #[serde(rename = "archivedRestorePoint")]
    pub r#archived_restore_point: Box<super::super::types::backup::PolicyVmTieringPolicyArchivedRestorePoint>,
}
