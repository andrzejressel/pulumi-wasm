#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyRestorePolicy {
    /// May only be set to true. If set, then the default Policy is restored.
    #[builder(into)]
    #[serde(rename = "default")]
    pub r#default: Box<bool>,
}
