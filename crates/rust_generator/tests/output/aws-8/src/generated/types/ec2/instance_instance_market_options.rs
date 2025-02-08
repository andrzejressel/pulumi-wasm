#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceInstanceMarketOptions {
    /// Type of market for the instance. Valid values are `spot` and `capacity-block`. Defaults to `spot`. Required if `spot_options` is specified.
    #[builder(into, default)]
    #[serde(rename = "marketType")]
    pub r#market_type: Box<Option<String>>,
    /// Block to configure the options for Spot Instances. See Spot Options below for details on attributes.
    #[builder(into, default)]
    #[serde(rename = "spotOptions")]
    pub r#spot_options: Box<Option<super::super::types::ec2::InstanceInstanceMarketOptionsSpotOptions>>,
}
