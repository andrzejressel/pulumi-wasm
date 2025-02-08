#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceSourceConfigurationImageRepositoryImageConfiguration {
    /// Port that your application listens to in the container. Defaults to `"8080"`.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<String>>,
    /// Secrets and parameters available to your service as environment variables. A map of key/value pairs, where the key is the desired name of the Secret in the environment (i.e. it does not have to match the name of the secret in Secrets Manager or SSM Parameter Store), and the value is the ARN of the secret from AWS Secrets Manager or the ARN of the parameter in AWS SSM Parameter Store.
    #[builder(into, default)]
    #[serde(rename = "runtimeEnvironmentSecrets")]
    pub r#runtime_environment_secrets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Environment variables available to your running App Runner service. A map of key/value pairs. Keys with a prefix of `AWSAPPRUNNER` are reserved for system use and aren't valid.
    #[builder(into, default)]
    #[serde(rename = "runtimeEnvironmentVariables")]
    pub r#runtime_environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// Command App Runner runs to start the application in the source image. If specified, this command overrides the Docker image’s default start command.
    #[builder(into, default)]
    #[serde(rename = "startCommand")]
    pub r#start_command: Box<Option<String>>,
}
