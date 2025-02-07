#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ObjectAccessControlProjectTeam {
    /// The project team associated with the entity
    #[builder(into, default)]
    #[serde(rename = "projectNumber")]
    pub r#project_number: Box<Option<String>>,
    /// The team.
    /// Possible values are: `editors`, `owners`, `viewers`.
    #[builder(into, default)]
    #[serde(rename = "team")]
    pub r#team: Box<Option<String>>,
}
