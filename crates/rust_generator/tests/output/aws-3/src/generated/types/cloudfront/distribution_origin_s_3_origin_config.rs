#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionOriginS3OriginConfig {
    /// The CloudFront origin access identity to associate with the origin.
    #[builder(into)]
    #[serde(rename = "originAccessIdentity")]
    pub r#origin_access_identity: Box<String>,
}
