#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceInstanceMarketOptionsSpotOptions {
    /// The behavior when a Spot Instance is interrupted. Valid values include `hibernate`, `stop`, `terminate` . The default is `terminate`.
    #[builder(into, default)]
    #[serde(rename = "instanceInterruptionBehavior")]
    pub r#instance_interruption_behavior: Box<Option<String>>,
    /// The maximum hourly price that you're willing to pay for a Spot Instance.
    #[builder(into, default)]
    #[serde(rename = "maxPrice")]
    pub r#max_price: Box<Option<String>>,
    /// The Spot Instance request type. Valid values include `one-time`, `persistent`. Persistent Spot Instance requests are only supported when the instance interruption behavior is either hibernate or stop. The default is `one-time`.
    #[builder(into, default)]
    #[serde(rename = "spotInstanceType")]
    pub r#spot_instance_type: Box<Option<String>>,
    /// The end date of the request, in UTC format (YYYY-MM-DDTHH:MM:SSZ). Supported only for persistent requests.
    #[builder(into, default)]
    #[serde(rename = "validUntil")]
    pub r#valid_until: Box<Option<String>>,
}
