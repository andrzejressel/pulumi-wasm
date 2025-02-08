#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiFeatureOnlineStoreFeatureviewFeatureRegistrySourceFeatureGroup {
    /// Identifier of the feature group.
    #[builder(into)]
    #[serde(rename = "featureGroupId")]
    pub r#feature_group_id: Box<String>,
    /// Identifiers of features under the feature group.
    #[builder(into)]
    #[serde(rename = "featureIds")]
    pub r#feature_ids: Box<Vec<String>>,
}
