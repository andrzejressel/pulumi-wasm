#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppLogs {
    /// A `application_logs` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "applicationLogs")]
    pub r#application_logs: Box<Option<super::super::types::appservice::WindowsWebAppLogsApplicationLogs>>,
    /// Should detailed error messages be enabled.
    #[builder(into, default)]
    #[serde(rename = "detailedErrorMessages")]
    pub r#detailed_error_messages: Box<Option<bool>>,
    /// Should tracing be enabled for failed requests.
    #[builder(into, default)]
    #[serde(rename = "failedRequestTracing")]
    pub r#failed_request_tracing: Box<Option<bool>>,
    /// A `http_logs` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "httpLogs")]
    pub r#http_logs: Box<Option<super::super::types::appservice::WindowsWebAppLogsHttpLogs>>,
}
