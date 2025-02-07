#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceHttpConfigAuthorizationConfigAwsIamConfig {
    /// Signing Amazon Web Services Region for IAM authorization.
    #[builder(into, default)]
    #[serde(rename = "signingRegion")]
    pub r#signing_region: Box<Option<String>>,
    /// Signing service name for IAM authorization.
    #[builder(into, default)]
    #[serde(rename = "signingServiceName")]
    pub r#signing_service_name: Box<Option<String>>,
}
