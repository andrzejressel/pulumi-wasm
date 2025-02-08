#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetOrganizationalUnitsChild {
    /// ARN of the organizational unit
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Parent identifier of the organizational units.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Name of the organizational unit
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
