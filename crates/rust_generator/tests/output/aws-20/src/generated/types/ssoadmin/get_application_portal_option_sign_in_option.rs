#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationPortalOptionSignInOption {
    #[builder(into)]
    #[serde(rename = "applicationUrl")]
    pub r#application_url: Box<String>,
    #[builder(into)]
    #[serde(rename = "origin")]
    pub r#origin: Box<String>,
}
