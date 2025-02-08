#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AiFeatureOnlineStoreFeatureviewFeatureRegistrySource {
    /// List of features that need to be synced to Online Store.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "featureGroups")]
    pub r#feature_groups: Box<Vec<super::super::types::vertex::AiFeatureOnlineStoreFeatureviewFeatureRegistrySourceFeatureGroup>>,
    /// The project number of the parent project of the feature Groups.
    #[builder(into, default)]
    #[serde(rename = "projectNumber")]
    pub r#project_number: Box<Option<String>>,
}
