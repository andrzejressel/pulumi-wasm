#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetServiceTemplateContainerResource {
    /// Determines whether CPU is only allocated during requests. True by default if the parent 'resources' field is not set. However, if
    /// 'resources' is set, this field must be explicitly set to true to preserve the default behavior.
    #[builder(into)]
    #[serde(rename = "cpuIdle")]
    pub r#cpu_idle: Box<bool>,
    /// Only memory, CPU, and nvidia.com/gpu are supported. Use key 'cpu' for CPU limit, 'memory' for memory limit, 'nvidia.com/gpu' for gpu limit. Note: The only supported values for CPU are '1', '2', '4', and '8'. Setting 4 CPU requires at least 2Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go
    #[builder(into)]
    #[serde(rename = "limits")]
    pub r#limits: Box<std::collections::HashMap<String, String>>,
    /// Determines whether CPU should be boosted on startup of a new container instance above the requested CPU threshold, this can help reduce cold-start latency.
    #[builder(into)]
    #[serde(rename = "startupCpuBoost")]
    pub r#startup_cpu_boost: Box<bool>,
}
