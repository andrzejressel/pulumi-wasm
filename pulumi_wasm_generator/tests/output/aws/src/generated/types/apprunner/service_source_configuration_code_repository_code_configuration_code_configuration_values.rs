#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues {
    /// Command App Runner runs to build your application.
    #[builder(into, default)]
    #[serde(rename = "buildCommand")]
    pub r#build_command: Box<Option<String>>,
    /// Port that your application listens to in the container. Defaults to `"8080"`.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<String>>,
    /// Runtime environment type for building and running an App Runner service. Represents a programming language runtime. Valid values: `PYTHON_3`, `NODEJS_12`, `NODEJS_14`, `NODEJS_16`, `CORRETTO_8`, `CORRETTO_11`, `GO_1`, `DOTNET_6`, `PHP_81`, `RUBY_31`.
    #[builder(into)]
    #[serde(rename = "runtime")]
    pub r#runtime: Box<String>,
    /// Secrets and parameters available to your service as environment variables. A map of key/value pairs, where the key is the desired name of the Secret in the environment (i.e. it does not have to match the name of the secret in Secrets Manager or SSM Parameter Store), and the value is the ARN of the secret from AWS Secrets Manager or the ARN of the parameter in AWS SSM Parameter Store.
    #[builder(into, default)]
    #[serde(rename = "runtimeEnvironmentSecrets")]
    pub r#runtime_environment_secrets: Box<Option<std::collections::HashMap<String, String>>>,
    /// Environment variables available to your running App Runner service. A map of key/value pairs. Keys with a prefix of `AWSAPPRUNNER` are reserved for system use and aren't valid.
    #[builder(into, default)]
    #[serde(rename = "runtimeEnvironmentVariables")]
    pub r#runtime_environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// Command App Runner runs to start your application.
    #[builder(into, default)]
    #[serde(rename = "startCommand")]
    pub r#start_command: Box<Option<String>>,
}
