#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolWindow {
    /// Whether automatic updates are enabled on the virtual machine. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enableAutomaticUpdates")]
    pub r#enable_automatic_updates: Box<Option<bool>>,
}
