#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatabaseInstanceSettingsIpConfigurationPscConfigPscAutoConnection {
    /// "The consumer network of this consumer endpoint. This must be a resource path that includes both the host project and the network name. For example, `projects/project1/global/networks/network1`. The consumer host project of this network might be different from the consumer service project."
    #[builder(into)]
    #[serde(rename = "consumerNetwork")]
    pub r#consumer_network: Box<String>,
    /// The project ID of consumer service project of this consumer endpoint.
    #[builder(into, default)]
    #[serde(rename = "consumerServiceProjectId")]
    pub r#consumer_service_project_id: Box<Option<String>>,
}
