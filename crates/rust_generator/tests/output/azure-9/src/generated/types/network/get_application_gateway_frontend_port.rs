#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewayFrontendPort {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Custom port which is used for probing the backend servers.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
