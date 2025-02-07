#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterWorkloadAutoscalerProfile {
    /// Specifies whether KEDA Autoscaler can be used for workloads.
    #[builder(into, default)]
    #[serde(rename = "kedaEnabled")]
    pub r#keda_enabled: Box<Option<bool>>,
    /// Specifies whether Vertical Pod Autoscaler should be enabled.
    /// 
    /// > **Note:** This requires that the Preview Feature `Microsoft.ContainerService/AKS-VPAPreview` is enabled and the Resource Provider is re-registered, see the documentation for more information.
    #[builder(into, default)]
    #[serde(rename = "verticalPodAutoscalerEnabled")]
    pub r#vertical_pod_autoscaler_enabled: Box<Option<bool>>,
}
