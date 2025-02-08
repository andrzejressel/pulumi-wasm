#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserProfileDetail {
    #[builder(into)]
    #[serde(rename = "iams")]
    pub r#iams: Box<Vec<super::super::types::datazone::UserProfileDetailIam>>,
    #[builder(into)]
    #[serde(rename = "ssos")]
    pub r#ssos: Box<Vec<super::super::types::datazone::UserProfileDetailSso>>,
}
