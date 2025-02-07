#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MaintenanceWindowTaskTaskInvocationParametersLambdaParameters {
    /// Pass client-specific information to the Lambda function that you are invoking.
    #[builder(into, default)]
    #[serde(rename = "clientContext")]
    pub r#client_context: Box<Option<String>>,
    /// JSON to provide to your Lambda function as input.
    #[builder(into, default)]
    #[serde(rename = "payload")]
    pub r#payload: Box<Option<String>>,
    /// Specify a Lambda function version or alias name.
    #[builder(into, default)]
    #[serde(rename = "qualifier")]
    pub r#qualifier: Box<Option<String>>,
}
