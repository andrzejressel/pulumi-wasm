#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTrafficPolicyDocumentRule {
    /// Configuration block for when you add a geoproximity rule, you configure Amazon Route 53 to route traffic to your resources based on the geographic location of your resources. Only valid for `geoproximity` type. See below
    #[builder(into, default)]
    #[serde(rename = "geoProximityLocations")]
    pub r#geo_proximity_locations: Box<Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleGeoProximityLocation>>>,
    /// ID of a rule you want to assign.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Configuration block for when you add a multivalue answer rule, you configure your traffic policy to route traffic approximately randomly to your healthy resources.  Only valid for `multivalue` type. See below
    #[builder(into, default)]
    #[serde(rename = "items")]
    pub r#items: Box<Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleItem>>>,
    /// Configuration block for when you add a geolocation rule, you configure your traffic policy to route your traffic based on the geographic location of your users.  Only valid for `geo` type. See below
    #[builder(into, default)]
    #[serde(rename = "locations")]
    pub r#locations: Box<Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleLocation>>>,
    /// Configuration block for the settings for the rule or endpoint that you want to route traffic to whenever the corresponding resources are available. Only valid for `failover` type. See below
    #[builder(into, default)]
    #[serde(rename = "primary")]
    pub r#primary: Box<Option<super::super::types::route53::GetTrafficPolicyDocumentRulePrimary>>,
    #[builder(into, default)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleRegion>>>,
    /// Configuration block for the rule or endpoint that you want to route traffic to whenever the primary resources are not available. Only valid for `failover` type. See below
    #[builder(into, default)]
    #[serde(rename = "secondary")]
    pub r#secondary: Box<Option<super::super::types::route53::GetTrafficPolicyDocumentRuleSecondary>>,
    /// Type of the rule.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
