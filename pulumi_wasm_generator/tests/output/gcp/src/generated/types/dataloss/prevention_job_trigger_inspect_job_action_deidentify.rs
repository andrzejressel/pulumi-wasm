#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobActionDeidentify {
    /// User settable Cloud Storage bucket and folders to store de-identified files.
    /// This field must be set for cloud storage deidentification.
    /// The output Cloud Storage bucket must be different from the input bucket.
    /// De-identified files will overwrite files in the output path.
    /// Form of: gs://bucket/folder/ or gs://bucket
    #[builder(into)]
    #[serde(rename = "cloudStorageOutput")]
    pub r#cloud_storage_output: Box<String>,
    /// List of user-specified file type groups to transform. If specified, only the files with these filetypes will be transformed.
    /// If empty, all supported files will be transformed. Supported types may be automatically added over time.
    /// If a file type is set in this field that isn't supported by the Deidentify action then the job will fail and will not be successfully created/started.
    /// Each value may be one of: `IMAGE`, `TEXT_FILE`, `CSV`, `TSV`.
    #[builder(into, default)]
    #[serde(rename = "fileTypesToTransforms")]
    pub r#file_types_to_transforms: Box<Option<Vec<String>>>,
    /// User specified deidentify templates and configs for structured, unstructured, and image files.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "transformationConfig")]
    pub r#transformation_config: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobActionDeidentifyTransformationConfig>>,
    /// Config for storing transformation details.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "transformationDetailsStorageConfig")]
    pub r#transformation_details_storage_config: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobActionDeidentifyTransformationDetailsStorageConfig>>,
}
