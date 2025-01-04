#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetFilterCloudStorageResourceReference {
    /// The bucket to scan.
    #[builder(into, default)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<Option<String>>,
    /// If within a project-level config, then this must match the config's project id.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<String>>,
}
