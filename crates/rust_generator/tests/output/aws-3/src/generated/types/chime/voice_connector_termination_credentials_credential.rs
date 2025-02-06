#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VoiceConnectorTerminationCredentialsCredential {
    /// RFC2617 compliant password associated with the SIP credentials.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// RFC2617 compliant username associated with the SIP credentials.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
