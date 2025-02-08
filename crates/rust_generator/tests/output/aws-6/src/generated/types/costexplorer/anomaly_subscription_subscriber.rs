#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AnomalySubscriptionSubscriber {
    /// The address of the subscriber. If type is `SNS`, this will be the arn of the sns topic. If type is `EMAIL`, this will be the destination email address.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// The type of subscription. Valid Values: `SNS` | `EMAIL`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
