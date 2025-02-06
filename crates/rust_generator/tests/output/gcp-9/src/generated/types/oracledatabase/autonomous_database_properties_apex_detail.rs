#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutonomousDatabasePropertiesApexDetail {
    /// The Oracle APEX Application Development version.
    #[builder(into, default)]
    #[serde(rename = "apexVersion")]
    pub r#apex_version: Box<Option<String>>,
    /// The Oracle REST Data Services (ORDS) version.
    #[builder(into, default)]
    #[serde(rename = "ordsVersion")]
    pub r#ords_version: Box<Option<String>>,
}
