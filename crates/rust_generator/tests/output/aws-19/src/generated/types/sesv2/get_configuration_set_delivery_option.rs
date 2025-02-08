#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetConfigurationSetDeliveryOption {
    /// The maximum amount of time, in seconds, that Amazon SES API v2 will attempt delivery of email. If specified, the value must greater than or equal to 300 seconds (5 minutes) and less than or equal to 50400 seconds (840 minutes).
    #[builder(into)]
    #[serde(rename = "maxDeliverySeconds")]
    pub r#max_delivery_seconds: Box<i32>,
    /// The name of the dedicated IP pool to associate with the configuration set.
    #[builder(into)]
    #[serde(rename = "sendingPoolName")]
    pub r#sending_pool_name: Box<String>,
    /// Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS).
    #[builder(into)]
    #[serde(rename = "tlsPolicy")]
    pub r#tls_policy: Box<String>,
}
