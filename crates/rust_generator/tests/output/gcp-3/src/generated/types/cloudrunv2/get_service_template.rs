#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplate {
    /// Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.
    /// 
    /// Cloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.
    /// All system annotations in v1 now have a corresponding field in v2 RevisionTemplate.
    /// 
    /// This field follows Kubernetes annotations' namespacing, limits, and rules.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: Box<std::collections::HashMap<String, String>>,
    /// Holds the containers that define the unit of execution for this Service.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateContainer>>,
    /// A reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek
    #[builder(into)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: Box<String>,
    /// The sandbox environment to host this Revision. Possible values: ["EXECUTION_ENVIRONMENT_GEN1", "EXECUTION_ENVIRONMENT_GEN2"]
    #[builder(into)]
    #[serde(rename = "executionEnvironment")]
    pub r#execution_environment: Box<String>,
    /// Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc.
    /// For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.
    /// 
    /// Cloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.
    /// All system labels in v1 now have a corresponding field in v2 RevisionTemplate.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// Sets the maximum number of requests that each serving instance can receive.
    /// If not specified or 0, defaults to 80 when requested CPU >= 1 and defaults to 1 when requested CPU < 1.
    #[builder(into)]
    #[serde(rename = "maxInstanceRequestConcurrency")]
    pub r#max_instance_request_concurrency: Box<i32>,
    /// Node Selector describes the hardware requirements of the resources.
    #[builder(into)]
    #[serde(rename = "nodeSelectors")]
    pub r#node_selectors: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateNodeSelector>>,
    /// The unique name for the revision. If this field is omitted, it will be automatically generated based on the Service name.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Box<String>,
    /// Scaling settings for this Revision.
    #[builder(into)]
    #[serde(rename = "scalings")]
    pub r#scalings: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateScaling>>,
    /// Email address of the IAM service account associated with the revision of the service. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<String>,
    /// Enables Cloud Service Mesh for this Revision.
    #[builder(into)]
    #[serde(rename = "serviceMeshes")]
    pub r#service_meshes: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateServiceMesh>>,
    /// Enables session affinity. For more information, go to https://cloud.google.com/run/docs/configuring/session-affinity
    #[builder(into)]
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Box<bool>,
    /// Max allowed time for an instance to respond to a request.
    /// 
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<String>,
    /// A list of Volumes to make available to containers.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateVolume>>,
    /// VPC Access configuration to use for this Task. For more information, visit https://cloud.google.com/run/docs/configuring/connecting-vpc.
    #[builder(into)]
    #[serde(rename = "vpcAccesses")]
    pub r#vpc_accesses: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateVpcAccess>>,
}
