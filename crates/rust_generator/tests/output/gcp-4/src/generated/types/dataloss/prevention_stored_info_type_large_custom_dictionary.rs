#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionStoredInfoTypeLargeCustomDictionary {
    /// Field in a BigQuery table where each cell represents a dictionary phrase.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "bigQueryField")]
    pub r#big_query_field: Box<Option<super::super::types::dataloss::PreventionStoredInfoTypeLargeCustomDictionaryBigQueryField>>,
    /// Set of files containing newline-delimited lists of dictionary phrases.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudStorageFileSet")]
    pub r#cloud_storage_file_set: Box<Option<super::super::types::dataloss::PreventionStoredInfoTypeLargeCustomDictionaryCloudStorageFileSet>>,
    /// Location to store dictionary artifacts in Google Cloud Storage. These files will only be accessible by project owners and the DLP API.
    /// If any of these artifacts are modified, the dictionary is considered invalid and can no longer be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "outputPath")]
    pub r#output_path: Box<super::super::types::dataloss::PreventionStoredInfoTypeLargeCustomDictionaryOutputPath>,
}
