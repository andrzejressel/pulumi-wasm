#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionProfileAlloydbSettings {
    /// Required. Input only. Initial user to setup during cluster creation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "initialUser")]
    pub r#initial_user: Box<super::super::types::databasemigrationservice::ConnectionProfileAlloydbSettingsInitialUser>,
    /// Labels for the AlloyDB cluster created by DMS.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Settings for the cluster's primary instance
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "primaryInstanceSettings")]
    pub r#primary_instance_settings: Box<Option<super::super::types::databasemigrationservice::ConnectionProfileAlloydbSettingsPrimaryInstanceSettings>>,
    /// Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster.
    /// It is specified in the form: 'projects/{project_number}/global/networks/{network_id}'. This is required to create a cluster.
    #[builder(into)]
    #[serde(rename = "vpcNetwork")]
    pub r#vpc_network: Box<String>,
}
