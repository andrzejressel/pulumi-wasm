#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewayProbeMatch {
    /// A snippet from the Response Body which must be present in the Response.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Box<String>,
    /// Status code of the application gateway custom error.
    #[builder(into)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Box<Vec<String>>,
}
