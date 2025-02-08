#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchTemplateInstanceMarketOptions {
    /// The market type. Can be `spot`.
    #[builder(into, default)]
    #[serde(rename = "marketType")]
    pub r#market_type: Box<Option<String>>,
    /// The options for [Spot Instance](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-spot-instances.html)
    #[builder(into, default)]
    #[serde(rename = "spotOptions")]
    pub r#spot_options: Box<Option<super::super::types::ec2::LaunchTemplateInstanceMarketOptionsSpotOptions>>,
}
