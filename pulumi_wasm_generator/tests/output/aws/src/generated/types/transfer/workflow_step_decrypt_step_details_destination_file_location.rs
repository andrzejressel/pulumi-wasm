#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowStepDecryptStepDetailsDestinationFileLocation {
    /// Specifies the details for the EFS file being copied.
    #[builder(into, default)]
    #[serde(rename = "efsFileLocation")]
    pub r#efs_file_location: Box<Option<super::super::types::transfer::WorkflowStepDecryptStepDetailsDestinationFileLocationEfsFileLocation>>,
    /// Specifies the details for the S3 file being copied.
    #[builder(into, default)]
    #[serde(rename = "s3FileLocation")]
    pub r#s_3_file_location: Box<Option<super::super::types::transfer::WorkflowStepDecryptStepDetailsDestinationFileLocationS3FileLocation>>,
}