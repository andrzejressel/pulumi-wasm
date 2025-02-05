#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClassificationJobUserPausedDetail {
    #[builder(into, default)]
    #[serde(rename = "jobExpiresAt")]
    pub r#job_expires_at: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "jobImminentExpirationHealthEventArn")]
    pub r#job_imminent_expiration_health_event_arn: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "jobPausedAt")]
    pub r#job_paused_at: Box<Option<String>>,
}
