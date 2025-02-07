#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleAppVersionFlexibleRuntimeSettings {
    /// Operating System of the application runtime.
    #[builder(into, default)]
    #[serde(rename = "operatingSystem")]
    pub r#operating_system: Box<Option<String>>,
    /// The runtime version of an App Engine flexible application.
    #[builder(into, default)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Box<Option<String>>,
}
