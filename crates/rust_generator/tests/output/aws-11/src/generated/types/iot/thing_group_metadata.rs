#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThingGroupMetadata {
    #[builder(into, default)]
    #[serde(rename = "creationDate")]
    pub r#creation_date: Box<Option<String>>,
    /// The name of the parent Thing Group.
    #[builder(into, default)]
    #[serde(rename = "parentGroupName")]
    pub r#parent_group_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "rootToParentGroups")]
    pub r#root_to_parent_groups: Box<Option<Vec<super::super::types::iot::ThingGroupMetadataRootToParentGroup>>>,
}
