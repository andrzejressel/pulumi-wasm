#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowStepDecryptStepDetails {
    /// Specifies the location for the file being copied. Use ${Transfer:username} in this field to parametrize the destination prefix by username.
    #[builder(into, default)]
    #[serde(rename = "destinationFileLocation")]
    pub r#destination_file_location: Box<Option<super::super::types::transfer::WorkflowStepDecryptStepDetailsDestinationFileLocation>>,
    /// The name of the step, used as an identifier.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// A flag that indicates whether or not to overwrite an existing file of the same name. The default is `FALSE`. Valid values are `TRUE` and `FALSE`.
    #[builder(into, default)]
    #[serde(rename = "overwriteExisting")]
    pub r#overwrite_existing: Box<Option<String>>,
    /// Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
    #[builder(into, default)]
    #[serde(rename = "sourceFileLocation")]
    pub r#source_file_location: Box<Option<String>>,
    /// The type of encryption used. Currently, this value must be `"PGP"`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
