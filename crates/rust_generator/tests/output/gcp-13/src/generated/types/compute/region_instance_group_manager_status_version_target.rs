#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionInstanceGroupManagerStatusVersionTarget {
    /// A bit indicating whether version target has been reached in this managed instance group, i.e. all instances are in their target version. Instances' target version are specified by version field on Instance Group Manager.
    #[builder(into, default)]
    #[serde(rename = "isReached")]
    pub r#is_reached: Box<Option<bool>>,
}
