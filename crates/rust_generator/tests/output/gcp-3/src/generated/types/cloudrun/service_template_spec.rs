#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceTemplateSpec {
    /// ContainerConcurrency specifies the maximum allowed in-flight (concurrent)
    /// requests per container of the Revision. If not specified or 0, defaults to 80 when
    /// requested CPU >= 1 and defaults to 1 when requested CPU < 1.
    #[builder(into, default)]
    #[serde(rename = "containerConcurrency")]
    pub r#container_concurrency: Box<Option<i32>>,
    /// Containers defines the unit of execution for this Revision.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainer>>>,
    /// Node Selector describes the hardware requirements of the resources.
    /// Use the following node selector keys to configure features on a Revision:
    /// - `run.googleapis.com/accelerator` sets the [type of GPU](https://cloud.google.com/run/docs/configuring/services/gpu) required by the Revision to run.
    #[builder(into, default)]
    #[serde(rename = "nodeSelector")]
    pub r#node_selector: Box<Option<std::collections::HashMap<String, String>>>,
    /// Email address of the IAM service account associated with the revision of the
    /// service. The service account represents the identity of the running revision,
    /// and determines what permissions the revision has. If not provided, the revision
    /// will use the project's default service account.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountName")]
    pub r#service_account_name: Box<Option<String>>,
    /// (Output, Deprecated)
    /// ServingState holds a value describing the state the resources
    /// are in for this Revision.
    /// It is expected
    /// that the system will manipulate this based on routability and load.
    /// 
    /// > **Warning:** `serving_state` is deprecated and will be removed in a future major release. This field is not supported by the Cloud Run API.
    #[builder(into, default)]
    #[serde(rename = "servingState")]
    pub r#serving_state: Box<Option<String>>,
    /// TimeoutSeconds holds the max duration the instance is allowed for responding to a request.
    #[builder(into, default)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<Option<i32>>,
    /// Volume represents a named volume in a container.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecVolume>>>,
}
