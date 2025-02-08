#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainSoftwareUpdateOptions {
    /// Whether automatic service software updates are enabled for the domain. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "autoSoftwareUpdateEnabled")]
    pub r#auto_software_update_enabled: Box<Option<bool>>,
}
