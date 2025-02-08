#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NodeGroupShareSettings {
    /// A map of project id and project config. This is only valid when shareType's value is SPECIFIC_PROJECTS.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "projectMaps")]
    pub r#project_maps: Box<Option<Vec<super::super::types::compute::NodeGroupShareSettingsProjectMap>>>,
    /// Node group sharing type.
    /// Possible values are: `ORGANIZATION`, `SPECIFIC_PROJECTS`, `LOCAL`.
    #[builder(into)]
    #[serde(rename = "shareType")]
    pub r#share_type: Box<String>,
}
