#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNames {
    /// An `application_server` block as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "applicationServer")]
    pub r#application_server: Box<Option<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServer>>,
    /// A `central_server` block as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "centralServer")]
    pub r#central_server: Box<Option<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServer>>,
    /// A `database_server` block as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "databaseServer")]
    pub r#database_server: Box<Option<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesDatabaseServer>>,
    /// A `shared_storage` block as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "sharedStorage")]
    pub r#shared_storage: Box<Option<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesSharedStorage>>,
}
