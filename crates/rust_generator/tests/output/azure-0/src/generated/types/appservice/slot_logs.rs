#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SlotLogs {
    /// An `application_logs` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "applicationLogs")]
    pub r#application_logs: Box<Option<super::super::types::appservice::SlotLogsApplicationLogs>>,
    /// Should `Detailed error messages` be enabled on this App Service slot? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "detailedErrorMessagesEnabled")]
    pub r#detailed_error_messages_enabled: Box<Option<bool>>,
    /// Should `Failed request tracing` be enabled on this App Service slot? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "failedRequestTracingEnabled")]
    pub r#failed_request_tracing_enabled: Box<Option<bool>>,
    /// An `http_logs` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "httpLogs")]
    pub r#http_logs: Box<Option<super::super::types::appservice::SlotLogsHttpLogs>>,
}
