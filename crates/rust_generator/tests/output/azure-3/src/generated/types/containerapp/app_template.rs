#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AppTemplate {
    /// One or more `azure_queue_scale_rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "azureQueueScaleRules")]
    pub r#azure_queue_scale_rules: Box<Option<Vec<super::super::types::containerapp::AppTemplateAzureQueueScaleRule>>>,
    /// One or more `container` blocks as detailed below.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Vec<super::super::types::containerapp::AppTemplateContainer>>,
    /// One or more `custom_scale_rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "customScaleRules")]
    pub r#custom_scale_rules: Box<Option<Vec<super::super::types::containerapp::AppTemplateCustomScaleRule>>>,
    /// One or more `http_scale_rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "httpScaleRules")]
    pub r#http_scale_rules: Box<Option<Vec<super::super::types::containerapp::AppTemplateHttpScaleRule>>>,
    /// The definition of an init container that is part of the group as documented in the `init_container` block below.
    #[builder(into, default)]
    #[serde(rename = "initContainers")]
    pub r#init_containers: Box<Option<Vec<super::super::types::containerapp::AppTemplateInitContainer>>>,
    /// The maximum number of replicas for this container.
    #[builder(into, default)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Box<Option<i32>>,
    /// The minimum number of replicas for this container.
    #[builder(into, default)]
    #[serde(rename = "minReplicas")]
    pub r#min_replicas: Box<Option<i32>>,
    /// The suffix for the revision. This value must be unique for the lifetime of the Resource. If omitted the service will use a hash function to create one.
    #[builder(into, default)]
    #[serde(rename = "revisionSuffix")]
    pub r#revision_suffix: Box<Option<String>>,
    /// One or more `tcp_scale_rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "tcpScaleRules")]
    pub r#tcp_scale_rules: Box<Option<Vec<super::super::types::containerapp::AppTemplateTcpScaleRule>>>,
    /// A `volume` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Option<Vec<super::super::types::containerapp::AppTemplateVolume>>>,
}
