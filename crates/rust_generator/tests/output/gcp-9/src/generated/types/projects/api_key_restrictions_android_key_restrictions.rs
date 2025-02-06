#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiKeyRestrictionsAndroidKeyRestrictions {
    /// A list of Android applications that are allowed to make API calls with this key.
    #[builder(into)]
    #[serde(rename = "allowedApplications")]
    pub r#allowed_applications: Box<Vec<super::super::types::projects::ApiKeyRestrictionsAndroidKeyRestrictionsAllowedApplication>>,
}
