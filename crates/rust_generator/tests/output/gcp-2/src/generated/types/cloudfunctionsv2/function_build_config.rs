#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionBuildConfig {
    /// Security patches are applied automatically to the runtime without requiring
    /// the function to be redeployed.
    #[builder(into, default)]
    #[serde(rename = "automaticUpdatePolicy")]
    pub r#automatic_update_policy: Box<Option<super::super::types::cloudfunctionsv2::FunctionBuildConfigAutomaticUpdatePolicy>>,
    /// (Output)
    /// The Cloud Build name of the latest successful
    /// deployment of the function.
    #[builder(into, default)]
    #[serde(rename = "build")]
    pub r#build: Box<Option<String>>,
    /// User managed repository created in Artifact Registry optionally with a customer managed encryption key.
    #[builder(into, default)]
    #[serde(rename = "dockerRepository")]
    pub r#docker_repository: Box<Option<String>>,
    /// The name of the function (as defined in source code) that will be executed.
    /// Defaults to the resource name suffix, if not specified. For backward
    /// compatibility, if function with given name is not found, then the system
    /// will try to use function named "function". For Node.js this is name of a
    /// function exported by the module specified in source_location.
    #[builder(into, default)]
    #[serde(rename = "entryPoint")]
    pub r#entry_point: Box<Option<String>>,
    /// User-provided build-time environment variables for the function.
    #[builder(into, default)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// Security patches are only applied when a function is redeployed.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "onDeployUpdatePolicy")]
    pub r#on_deploy_update_policy: Box<Option<super::super::types::cloudfunctionsv2::FunctionBuildConfigOnDeployUpdatePolicy>>,
    /// The runtime in which to run the function. Required when deploying a new
    /// function, optional when updating an existing function.
    #[builder(into, default)]
    #[serde(rename = "runtime")]
    pub r#runtime: Box<Option<String>>,
    /// The fully-qualified name of the service account to be used for building the container.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
    /// The location of the function source code.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<super::super::types::cloudfunctionsv2::FunctionBuildConfigSource>>,
    /// Name of the Cloud Build Custom Worker Pool that should be used to build the function.
    #[builder(into, default)]
    #[serde(rename = "workerPool")]
    pub r#worker_pool: Box<Option<String>>,
}
