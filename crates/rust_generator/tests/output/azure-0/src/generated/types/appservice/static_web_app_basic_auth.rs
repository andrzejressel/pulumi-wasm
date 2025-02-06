#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StaticWebAppBasicAuth {
    /// The Environment types to use the Basic Auth for access. Possible values include `AllEnvironments` and `StagingEnvironments`.
    #[builder(into)]
    #[serde(rename = "environments")]
    pub r#environments: Box<String>,
    /// The password for the basic authentication access.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
}
