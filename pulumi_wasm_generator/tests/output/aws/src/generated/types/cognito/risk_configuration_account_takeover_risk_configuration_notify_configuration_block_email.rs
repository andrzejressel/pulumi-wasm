#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationBlockEmail {
    /// The email HTML body.
    #[builder(into)]
    #[serde(rename = "htmlBody")]
    pub r#html_body: Box<String>,
    /// The email subject.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Box<String>,
    /// The email text body.
    #[builder(into)]
    #[serde(rename = "textBody")]
    pub r#text_body: Box<String>,
}