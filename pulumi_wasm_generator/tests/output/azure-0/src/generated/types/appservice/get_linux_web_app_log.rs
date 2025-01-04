#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxWebAppLog {
    /// A `application_logs` block as defined above.
    #[builder(into)]
    #[serde(rename = "applicationLogs")]
    pub r#application_logs: Box<Vec<super::super::types::appservice::GetLinuxWebAppLogApplicationLog>>,
    /// Is Detailed Error Messaging enabled.
    #[builder(into)]
    #[serde(rename = "detailedErrorMessages")]
    pub r#detailed_error_messages: Box<bool>,
    /// Is Failed Request Tracing enabled.
    #[builder(into)]
    #[serde(rename = "failedRequestTracing")]
    pub r#failed_request_tracing: Box<bool>,
    /// An `http_logs` block as defined above.
    #[builder(into)]
    #[serde(rename = "httpLogs")]
    pub r#http_logs: Box<Vec<super::super::types::appservice::GetLinuxWebAppLogHttpLog>>,
}
