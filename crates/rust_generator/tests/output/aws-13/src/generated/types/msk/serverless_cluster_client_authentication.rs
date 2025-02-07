#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServerlessClusterClientAuthentication {
    /// Details for client authentication using SASL. See below.
    #[builder(into)]
    #[serde(rename = "sasl")]
    pub r#sasl: Box<super::super::types::msk::ServerlessClusterClientAuthenticationSasl>,
}
