#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeSourceParametersRabbitmqBrokerParametersCredentials {
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into)]
    #[serde(rename = "basicAuth")]
    pub r#basic_auth: Box<String>,
}
