#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSetRegexFileSet {
    /// The name of a Cloud Storage bucket.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// A list of regular expressions matching file paths to exclude. All files in the bucket that match at
    /// least one of these regular expressions will be excluded from the scan.
    #[builder(into, default)]
    #[serde(rename = "excludeRegexes")]
    pub r#exclude_regexes: Box<Option<Vec<String>>>,
    /// A list of regular expressions matching file paths to include. All files in the bucket
    /// that match at least one of these regular expressions will be included in the set of files,
    /// except for those that also match an item in excludeRegex. Leaving this field empty will
    /// match all files by default (this is equivalent to including .* in the list)
    #[builder(into, default)]
    #[serde(rename = "includeRegexes")]
    pub r#include_regexes: Box<Option<Vec<String>>>,
}
