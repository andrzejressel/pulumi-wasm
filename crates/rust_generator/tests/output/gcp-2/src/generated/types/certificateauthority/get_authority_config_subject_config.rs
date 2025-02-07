#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAuthorityConfigSubjectConfig {
    /// The subject alternative name fields.
    #[builder(into)]
    #[serde(rename = "subjectAltNames")]
    pub r#subject_alt_names: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigSubjectConfigSubjectAltName>>,
    /// Contains distinguished name fields such as the location and organization.
    #[builder(into)]
    #[serde(rename = "subjects")]
    pub r#subjects: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigSubjectConfigSubject>>,
}
