#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LiteSubscriptionDeliveryConfig {
    /// When this subscription should send messages to subscribers relative to messages persistence in storage.
    /// Possible values are: `DELIVER_IMMEDIATELY`, `DELIVER_AFTER_STORED`, `DELIVERY_REQUIREMENT_UNSPECIFIED`.
    #[builder(into)]
    #[serde(rename = "deliveryRequirement")]
    pub r#delivery_requirement: Box<String>,
}
