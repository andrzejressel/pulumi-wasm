#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFunctionBuildConfig {
    /// Security patches are applied automatically to the runtime without requiring
    /// the function to be redeployed.
    #[builder(into)]
    #[serde(rename = "automaticUpdatePolicies")]
    pub r#automatic_update_policies: Box<Vec<super::super::types::cloudfunctionsv2::GetFunctionBuildConfigAutomaticUpdatePolicy>>,
    /// The Cloud Build name of the latest successful
    /// deployment of the function.
    #[builder(into)]
    #[serde(rename = "build")]
    pub r#build: Box<String>,
    /// User managed repository created in Artifact Registry optionally with a customer managed encryption key.
    #[builder(into)]
    #[serde(rename = "dockerRepository")]
    pub r#docker_repository: Box<String>,
    /// The name of the function (as defined in source code) that will be executed.
    /// Defaults to the resource name suffix, if not specified. For backward
    /// compatibility, if function with given name is not found, then the system
    /// will try to use function named "function". For Node.js this is name of a
    /// function exported by the module specified in source_location.
    #[builder(into)]
    #[serde(rename = "entryPoint")]
    pub r#entry_point: Box<String>,
    /// User-provided build-time environment variables for the function.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<std::collections::HashMap<String, String>>,
    /// Security patches are only applied when a function is redeployed.
    #[builder(into)]
    #[serde(rename = "onDeployUpdatePolicies")]
    pub r#on_deploy_update_policies: Box<Vec<super::super::types::cloudfunctionsv2::GetFunctionBuildConfigOnDeployUpdatePolicy>>,
    /// The runtime in which to run the function. Required when deploying a new
    /// function, optional when updating an existing function.
    #[builder(into)]
    #[serde(rename = "runtime")]
    pub r#runtime: Box<String>,
    /// The fully-qualified name of the service account to be used for building the container.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<String>,
    /// The location of the function source code.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Box<Vec<super::super::types::cloudfunctionsv2::GetFunctionBuildConfigSource>>,
    /// Name of the Cloud Build Custom Worker Pool that should be used to build the function.
    #[builder(into)]
    #[serde(rename = "workerPool")]
    pub r#worker_pool: Box<String>,
}
