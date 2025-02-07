#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RecordGeoproximityRoutingPolicy {
    /// A AWS region where the resource is present.
    #[builder(into, default)]
    #[serde(rename = "awsRegion")]
    pub r#aws_region: Box<Option<String>>,
    /// Route more traffic or less traffic to the resource by specifying a value ranges between -90 to 90. See https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy-geoproximity.html for bias details.
    #[builder(into, default)]
    #[serde(rename = "bias")]
    pub r#bias: Box<Option<i32>>,
    /// Specify `latitude` and `longitude` for routing traffic to non-AWS resources.
    #[builder(into, default)]
    #[serde(rename = "coordinates")]
    pub r#coordinates: Box<Option<Vec<super::super::types::route53::RecordGeoproximityRoutingPolicyCoordinate>>>,
    /// A AWS local zone group where the resource is present. See https://docs.aws.amazon.com/local-zones/latest/ug/available-local-zones.html for local zone group list.
    #[builder(into, default)]
    #[serde(rename = "localZoneGroup")]
    pub r#local_zone_group: Box<Option<String>>,
}
