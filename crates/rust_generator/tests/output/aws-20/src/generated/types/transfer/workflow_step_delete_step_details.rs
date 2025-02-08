#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowStepDeleteStepDetails {
    /// The name of the step, used as an identifier.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
    #[builder(into, default)]
    #[serde(rename = "sourceFileLocation")]
    pub r#source_file_location: Box<Option<String>>,
}
