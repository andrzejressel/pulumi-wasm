#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPlanRuleCopyActionLifecycle {
    #[builder(into)]
    #[serde(rename = "coldStorageAfter")]
    pub r#cold_storage_after: Box<i32>,
    #[builder(into)]
    #[serde(rename = "deleteAfter")]
    pub r#delete_after: Box<i32>,
    #[builder(into)]
    #[serde(rename = "optInToArchiveForSupportedResources")]
    pub r#opt_in_to_archive_for_supported_resources: Box<bool>,
}
