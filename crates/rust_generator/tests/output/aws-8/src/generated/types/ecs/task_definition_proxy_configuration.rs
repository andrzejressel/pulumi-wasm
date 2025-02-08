#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskDefinitionProxyConfiguration {
    /// Name of the container that will serve as the App Mesh proxy.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<String>,
    /// Set of network configuration parameters to provide the Container Network Interface (CNI) plugin, specified a key-value mapping.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// Proxy type. The default value is `APPMESH`. The only supported value is `APPMESH`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
