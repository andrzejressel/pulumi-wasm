#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpotInstanceRequestLaunchTemplate {
    /// ID of the launch template. Conflicts with `name`.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Name of the launch template. Conflicts with `id`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Template version. Can be a specific version number, `$Latest` or `$Default`. The default value is `$Default`.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
