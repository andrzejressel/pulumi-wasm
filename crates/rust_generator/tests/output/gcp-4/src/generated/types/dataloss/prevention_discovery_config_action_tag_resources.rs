#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionDiscoveryConfigActionTagResources {
    /// Whether applying a tag to a resource should lower the risk of the profile for that resource. For example, in conjunction with an [IAM deny policy](https://cloud.google.com/iam/docs/deny-overview), you can deny all principals a permission if a tag value is present, mitigating the risk of the resource. This also lowers the data risk of resources at the lower levels of the resource hierarchy. For example, reducing the data risk of a table data profile also reduces the data risk of the constituent column data profiles.
    #[builder(into, default)]
    #[serde(rename = "lowerDataRiskToLow")]
    pub r#lower_data_risk_to_low: Box<Option<bool>>,
    /// The profile generations for which the tag should be attached to resources. If you attach a tag to only new profiles, then if the sensitivity score of a profile subsequently changes, its tag doesn't change. By default, this field includes only new profiles. To include both new and updated profiles for tagging, this field should explicitly include both `PROFILE_GENERATION_NEW` and `PROFILE_GENERATION_UPDATE`.
    /// Each value may be one of: `PROFILE_GENERATION_NEW`, `PROFILE_GENERATION_UPDATE`.
    #[builder(into, default)]
    #[serde(rename = "profileGenerationsToTags")]
    pub r#profile_generations_to_tags: Box<Option<Vec<String>>>,
    /// The tags to associate with different conditions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tagConditions")]
    pub r#tag_conditions: Box<Option<Vec<super::super::types::dataloss::PreventionDiscoveryConfigActionTagResourcesTagCondition>>>,
}
