#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiEndpointDeployedModel {
    /// (Output)
    /// A description of resources that to large degree are decided by Vertex AI, and require only a modest additional configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "automaticResources")]
    pub r#automatic_resources: Box<Option<Vec<super::super::types::vertex::AiEndpointDeployedModelAutomaticResource>>>,
    /// (Output)
    /// Output only. Timestamp when the DeployedModel was created.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// (Output)
    /// A description of resources that are dedicated to the DeployedModel, and that need a higher degree of manual configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dedicatedResources")]
    pub r#dedicated_resources: Box<Option<Vec<super::super::types::vertex::AiEndpointDeployedModelDedicatedResource>>>,
    /// Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// (Output)
    /// These logs are like standard server access logs, containing information like timestamp and latency for each prediction request. Note that Stackdriver logs may incur a cost, especially if your project receives prediction requests at a high queries per second rate (QPS). Estimate your costs before enabling this option.
    #[builder(into, default)]
    #[serde(rename = "enableAccessLogging")]
    pub r#enable_access_logging: Box<Option<bool>>,
    /// (Output)
    /// If true, the container of the DeployedModel instances will send `stderr` and `stdout` streams to Stackdriver Logging. Only supported for custom-trained Models and AutoML Tabular Models.
    #[builder(into, default)]
    #[serde(rename = "enableContainerLogging")]
    pub r#enable_container_logging: Box<Option<bool>>,
    /// (Output)
    /// The ID of the DeployedModel. If not provided upon deployment, Vertex AI will generate a value for this ID. This value should be 1-10 characters, and valid characters are /[0-9]/.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// (Output)
    /// The name of the Model that this is the deployment of. Note that the Model may be in a different location than the DeployedModel's Endpoint.
    #[builder(into, default)]
    #[serde(rename = "model")]
    pub r#model: Box<Option<String>>,
    /// (Output)
    /// Output only. The version ID of the model that is deployed.
    #[builder(into, default)]
    #[serde(rename = "modelVersionId")]
    pub r#model_version_id: Box<Option<String>>,
    /// (Output)
    /// Output only. Provide paths for users to send predict/explain/health requests directly to the deployed model services running on Cloud via private services access. This field is populated if network is configured.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "privateEndpoints")]
    pub r#private_endpoints: Box<Option<Vec<super::super::types::vertex::AiEndpointDeployedModelPrivateEndpoint>>>,
    /// (Output)
    /// The service account that the DeployedModel's container runs as. Specify the email address of the service account. If this service account is not specified, the container runs as a service account that doesn't have access to the resource project. Users deploying the Model must have the `iam.serviceAccounts.actAs` permission on this service account.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
    /// (Output)
    /// The resource name of the shared DeploymentResourcePool to deploy on. Format: projects/{project}/locations/{location}/deploymentResourcePools/{deployment_resource_pool}
    #[builder(into, default)]
    #[serde(rename = "sharedResources")]
    pub r#shared_resources: Box<Option<String>>,
}
