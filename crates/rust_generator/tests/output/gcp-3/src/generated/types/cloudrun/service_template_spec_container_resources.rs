#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateSpecContainerResources {
    /// Limits describes the maximum amount of compute resources allowed.
    /// The values of the map is string form of the 'quantity' k8s type:
    /// https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go
    #[builder(into, default)]
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<std::collections::HashMap<String, String>>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is
    /// explicitly specified, otherwise to an implementation-defined value.
    /// The values of the map is string form of the 'quantity' k8s type:
    /// https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go
    #[builder(into, default)]
    #[serde(rename = "requests")]
    pub r#requests: Box<Option<std::collections::HashMap<String, String>>>,
}
