#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateSpec {
    /// ContainerConcurrency specifies the maximum allowed in-flight (concurrent)
    /// requests per container of the Revision. If not specified or 0, defaults to 80 when
    /// requested CPU >= 1 and defaults to 1 when requested CPU < 1.
    #[builder(into)]
    #[serde(rename = "containerConcurrency")]
    pub r#container_concurrency: Box<i32>,
    /// Containers defines the unit of execution for this Revision.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainer>>,
    /// Node Selector describes the hardware requirements of the resources.
    /// Use the following node selector keys to configure features on a Revision:
    ///   - 'run.googleapis.com/accelerator' sets the [type of GPU](https://cloud.google.com/run/docs/configuring/services/gpu) required by the Revision to run.
    #[builder(into)]
    #[serde(rename = "nodeSelector")]
    pub r#node_selector: Box<std::collections::HashMap<String, String>>,
    /// Email address of the IAM service account associated with the revision of the
    /// service. The service account represents the identity of the running revision,
    /// and determines what permissions the revision has. If not provided, the revision
    /// will use the project's default service account.
    #[builder(into)]
    #[serde(rename = "serviceAccountName")]
    pub r#service_account_name: Box<String>,
    /// ServingState holds a value describing the state the resources
    /// are in for this Revision.
    /// It is expected
    /// that the system will manipulate this based on routability and load.
    #[builder(into)]
    #[serde(rename = "servingState")]
    pub r#serving_state: Box<String>,
    /// TimeoutSeconds holds the max duration the instance is allowed for responding to a request.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<i32>,
    /// Volume represents a named volume in a container.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecVolume>>,
}
