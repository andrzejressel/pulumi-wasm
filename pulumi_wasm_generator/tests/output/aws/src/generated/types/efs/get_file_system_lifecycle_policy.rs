#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFileSystemLifecyclePolicy {
    #[builder(into)]
    #[serde(rename = "transitionToArchive")]
    pub r#transition_to_archive: Box<String>,
    #[builder(into)]
    #[serde(rename = "transitionToIa")]
    pub r#transition_to_ia: Box<String>,
    #[builder(into)]
    #[serde(rename = "transitionToPrimaryStorageClass")]
    pub r#transition_to_primary_storage_class: Box<String>,
}