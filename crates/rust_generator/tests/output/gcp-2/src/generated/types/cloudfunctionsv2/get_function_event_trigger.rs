#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFunctionEventTrigger {
    /// Criteria used to filter events.
    #[builder(into)]
    #[serde(rename = "eventFilters")]
    pub r#event_filters: Box<Vec<super::super::types::cloudfunctionsv2::GetFunctionEventTriggerEventFilter>>,
    /// Required. The type of event to observe.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: Box<String>,
    /// The name of a Pub/Sub topic in the same project that will be used
    /// as the transport topic for the event delivery.
    #[builder(into)]
    #[serde(rename = "pubsubTopic")]
    pub r#pubsub_topic: Box<String>,
    /// Describes the retry policy in case of function's execution failure.
    /// Retried execution is charged as any other execution. Possible values: ["RETRY_POLICY_UNSPECIFIED", "RETRY_POLICY_DO_NOT_RETRY", "RETRY_POLICY_RETRY"]
    #[builder(into)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Box<String>,
    /// Optional. The email of the trigger's service account. The service account
    /// must have permission to invoke Cloud Run services. If empty, defaults to the
    /// Compute Engine default service account: {project_number}-compute@developer.gserviceaccount.com.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<String>,
    /// Output only. The resource name of the Eventarc trigger.
    #[builder(into)]
    #[serde(rename = "trigger")]
    pub r#trigger: Box<String>,
    /// The region that the trigger will be in. The trigger will only receive
    /// events originating in this region. It can be the same
    /// region as the function, a different region or multi-region, or the global
    /// region. If not provided, defaults to the same region as the function.
    #[builder(into)]
    #[serde(rename = "triggerRegion")]
    pub r#trigger_region: Box<String>,
}
