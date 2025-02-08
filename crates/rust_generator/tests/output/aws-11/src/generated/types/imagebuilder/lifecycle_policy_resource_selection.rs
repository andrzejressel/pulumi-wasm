#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LifecyclePolicyResourceSelection {
    /// A list of recipe that are used as selection criteria for the output images that the lifecycle policy applies to. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "recipes")]
    pub r#recipes: Box<Option<Vec<super::super::types::imagebuilder::LifecyclePolicyResourceSelectionRecipe>>>,
    /// A list of tags that are used as selection criteria for the Image Builder image resources that the lifecycle policy applies to.
    #[builder(into, default)]
    #[serde(rename = "tagMap")]
    pub r#tag_map: Box<Option<std::collections::HashMap<String, String>>>,
}
