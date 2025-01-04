#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PlanRuleCopyActionLifecycle {
    /// Specifies the number of days after creation that a recovery point is moved to cold storage.
    #[builder(into, default)]
    #[serde(rename = "coldStorageAfter")]
    pub r#cold_storage_after: Box<Option<i32>>,
    /// Specifies the number of days after creation that a recovery point is deleted. Must be 90 days greater than `cold_storage_after`.
    #[builder(into, default)]
    #[serde(rename = "deleteAfter")]
    pub r#delete_after: Box<Option<i32>>,
    /// This setting will instruct your backup plan to transition supported resources to archive (cold) storage tier in accordance with your lifecycle settings.
    #[builder(into, default)]
    #[serde(rename = "optInToArchiveForSupportedResources")]
    pub r#opt_in_to_archive_for_supported_resources: Box<Option<bool>>,
}
