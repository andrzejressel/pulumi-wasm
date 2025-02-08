#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVpnServerConfigurationRadiusServer {
    /// The Address of the Radius Server.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// The Score of the Radius Server determines the priority of the server.
    #[builder(into)]
    #[serde(rename = "score")]
    pub r#score: Box<i32>,
    /// The Secret used to communicate with the Radius Server.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Box<String>,
}
