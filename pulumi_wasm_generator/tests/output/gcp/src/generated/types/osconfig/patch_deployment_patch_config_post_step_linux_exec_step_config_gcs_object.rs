#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PatchDeploymentPatchConfigPostStepLinuxExecStepConfigGcsObject {
    /// Bucket of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change.
    #[builder(into)]
    #[serde(rename = "generationNumber")]
    pub r#generation_number: Box<String>,
    /// Name of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<String>,
}
