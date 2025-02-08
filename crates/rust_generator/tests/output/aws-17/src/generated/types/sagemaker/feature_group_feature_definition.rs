#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FeatureGroupFeatureDefinition {
    #[builder(into, default)]
    #[serde(rename = "collectionConfig")]
    pub r#collection_config: Box<Option<super::super::types::sagemaker::FeatureGroupFeatureDefinitionCollectionConfig>>,
    #[builder(into, default)]
    #[serde(rename = "collectionType")]
    pub r#collection_type: Box<Option<String>>,
    /// The name of a feature. `feature_name` cannot be any of the following: `is_deleted`, `write_time`, `api_invocation_time`.
    #[builder(into, default)]
    #[serde(rename = "featureName")]
    pub r#feature_name: Box<Option<String>>,
    /// The value type of a feature. Valid values are `Integral`, `Fractional`, or `String`.
    #[builder(into, default)]
    #[serde(rename = "featureType")]
    pub r#feature_type: Box<Option<String>>,
}
