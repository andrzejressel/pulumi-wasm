#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SubscriptionClientScopedSubscription {
    /// Specifies the Client ID of the application that created the client-scoped subscription. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** Client ID can be null or empty, but it must match the client ID set on the JMS client application. From the Azure Service Bus perspective, a null client ID and an empty client id have the same behavior. If the client ID is set to null or empty, it is only accessible to client applications whose client ID is also set to null or empty.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// Whether the client scoped subscription is durable. This property can only be controlled from the application side.
    #[builder(into, default)]
    #[serde(rename = "isClientScopedSubscriptionDurable")]
    pub r#is_client_scoped_subscription_durable: Box<Option<bool>>,
    /// Whether the client scoped subscription is shareable. Defaults to `true` Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "isClientScopedSubscriptionShareable")]
    pub r#is_client_scoped_subscription_shareable: Box<Option<bool>>,
}
