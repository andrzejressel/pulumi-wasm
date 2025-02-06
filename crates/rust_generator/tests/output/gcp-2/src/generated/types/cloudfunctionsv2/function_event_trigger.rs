#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionEventTrigger {
    /// Criteria used to filter events.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "eventFilters")]
    pub r#event_filters: Box<Option<Vec<super::super::types::cloudfunctionsv2::FunctionEventTriggerEventFilter>>>,
    /// Required. The type of event to observe.
    #[builder(into, default)]
    #[serde(rename = "eventType")]
    pub r#event_type: Box<Option<String>>,
    /// The name of a Pub/Sub topic in the same project that will be used
    /// as the transport topic for the event delivery.
    #[builder(into, default)]
    #[serde(rename = "pubsubTopic")]
    pub r#pubsub_topic: Box<Option<String>>,
    /// Describes the retry policy in case of function's execution failure.
    /// Retried execution is charged as any other execution.
    /// Possible values are: `RETRY_POLICY_UNSPECIFIED`, `RETRY_POLICY_DO_NOT_RETRY`, `RETRY_POLICY_RETRY`.
    #[builder(into, default)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Box<Option<String>>,
    /// Optional. The email of the trigger's service account. The service account
    /// must have permission to invoke Cloud Run services. If empty, defaults to the
    /// Compute Engine default service account: {project_number}-compute@developer.gserviceaccount.com.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<Option<String>>,
    /// (Output)
    /// Output only. The resource name of the Eventarc trigger.
    #[builder(into, default)]
    #[serde(rename = "trigger")]
    pub r#trigger: Box<Option<String>>,
    /// The region that the trigger will be in. The trigger will only receive
    /// events originating in this region. It can be the same
    /// region as the function, a different region or multi-region, or the global
    /// region. If not provided, defaults to the same region as the function.
    #[builder(into, default)]
    #[serde(rename = "triggerRegion")]
    pub r#trigger_region: Box<Option<String>>,
}
