#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppTemplate {
    #[builder(into)]
    #[serde(rename = "azureQueueScaleRules")]
    pub r#azure_queue_scale_rules: Box<Vec<super::super::types::containerapp::GetAppTemplateAzureQueueScaleRule>>,
    /// One or more `container` blocks as detailed below.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Vec<super::super::types::containerapp::GetAppTemplateContainer>>,
    #[builder(into, default)]
    #[serde(rename = "customScaleRules")]
    pub r#custom_scale_rules: Box<Option<Vec<super::super::types::containerapp::GetAppTemplateCustomScaleRule>>>,
    #[builder(into)]
    #[serde(rename = "httpScaleRules")]
    pub r#http_scale_rules: Box<Vec<super::super::types::containerapp::GetAppTemplateHttpScaleRule>>,
    /// One or more `init_container` blocks as detailed below.
    #[builder(into)]
    #[serde(rename = "initContainers")]
    pub r#init_containers: Box<Vec<super::super::types::containerapp::GetAppTemplateInitContainer>>,
    /// The maximum number of replicas for this container.
    #[builder(into)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Box<i32>,
    /// The minimum number of replicas for this container.
    #[builder(into)]
    #[serde(rename = "minReplicas")]
    pub r#min_replicas: Box<i32>,
    /// The suffix string to which this `traffic_weight` applies.
    #[builder(into)]
    #[serde(rename = "revisionSuffix")]
    pub r#revision_suffix: Box<String>,
    #[builder(into)]
    #[serde(rename = "tcpScaleRules")]
    pub r#tcp_scale_rules: Box<Vec<super::super::types::containerapp::GetAppTemplateTcpScaleRule>>,
    /// A `volume` block as detailed below.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Vec<super::super::types::containerapp::GetAppTemplateVolume>>,
}
