#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollectionIncludeRegexesPatternCloudStorageRegex {
    /// Regex to test the bucket name against. If empty, all buckets match. Example: "marketing2021" or "(marketing)\d{4}" will both match the bucket gs://marketing2021
    #[builder(into, default)]
    #[serde(rename = "bucketNameRegex")]
    pub r#bucket_name_regex: Box<Option<String>>,
    /// For organizations, if unset, will match all projects.
    #[builder(into, default)]
    #[serde(rename = "projectIdRegex")]
    pub r#project_id_regex: Box<Option<String>>,
}
