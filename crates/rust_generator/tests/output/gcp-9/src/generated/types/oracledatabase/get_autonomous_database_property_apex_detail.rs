#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAutonomousDatabasePropertyApexDetail {
    /// The Oracle APEX Application Development version.
    #[builder(into)]
    #[serde(rename = "apexVersion")]
    pub r#apex_version: Box<String>,
    /// The Oracle REST Data Services (ORDS) version.
    #[builder(into)]
    #[serde(rename = "ordsVersion")]
    pub r#ords_version: Box<String>,
}
