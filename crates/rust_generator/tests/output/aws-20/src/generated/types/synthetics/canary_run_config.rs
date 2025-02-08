#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CanaryRunConfig {
    /// Whether this canary is to use active AWS X-Ray tracing when it runs. You can enable active tracing only for canaries that use version syn-nodejs-2.0 or later for their canary runtime.
    #[builder(into, default)]
    #[serde(rename = "activeTracing")]
    pub r#active_tracing: Box<Option<bool>>,
    /// Map of environment variables that are accessible from the canary during execution. Please see [AWS Docs](https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html#configuration-envvars-runtime) for variables reserved for Lambda.
    #[builder(into, default)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// Maximum amount of memory available to the canary while it is running, in MB. The value you specify must be a multiple of 64.
    #[builder(into, default)]
    #[serde(rename = "memoryInMb")]
    pub r#memory_in_mb: Box<Option<i32>>,
    /// Number of seconds the canary is allowed to run before it must stop. If you omit this field, the frequency of the canary is used, up to a maximum of 840 (14 minutes).
    #[builder(into, default)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<Option<i32>>,
}
