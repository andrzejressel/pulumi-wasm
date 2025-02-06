#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatabaseInstancesInstanceSettingIpConfigurationPscConfigPscAutoConnection {
    /// The consumer network of this consumer endpoint. This must be a resource path that includes both the host project and the network name. The consumer host project of this network might be different from the consumer service project.
    #[builder(into)]
    #[serde(rename = "consumerNetwork")]
    pub r#consumer_network: Box<String>,
    /// The project ID of consumer service project of this consumer endpoint.
    #[builder(into)]
    #[serde(rename = "consumerServiceProjectId")]
    pub r#consumer_service_project_id: Box<String>,
}
