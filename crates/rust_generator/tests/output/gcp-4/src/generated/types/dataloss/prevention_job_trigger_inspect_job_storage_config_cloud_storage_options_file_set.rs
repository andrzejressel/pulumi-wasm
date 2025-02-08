#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet {
    /// The regex-filtered set of files to scan.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "regexFileSet")]
    pub r#regex_file_set: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSetRegexFileSet>>,
    /// The Cloud Storage url of the file(s) to scan, in the format `gs://<bucket>/<path>`. Trailing wildcard
    /// in the path is allowed.
    /// If the url ends in a trailing slash, the bucket or directory represented by the url will be scanned
    /// non-recursively (content in sub-directories will not be scanned). This means that `gs://mybucket/` is
    /// equivalent to `gs://mybucket/*`, and `gs://mybucket/directory/` is equivalent to `gs://mybucket/directory/*`.
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
}
