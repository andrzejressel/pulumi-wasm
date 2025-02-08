#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthorityConfigSubjectConfig {
    /// Contains distinguished name fields such as the location and organization.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Box<super::super::types::certificateauthority::AuthorityConfigSubjectConfigSubject>,
    /// The subject alternative name fields.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "subjectAltName")]
    pub r#subject_alt_name: Box<Option<super::super::types::certificateauthority::AuthorityConfigSubjectConfigSubjectAltName>>,
}
