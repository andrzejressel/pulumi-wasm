#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetConfigurationWindow {
    /// The duration of the maintenance window.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// Effective expiration date of the maintenance window.
    #[builder(into)]
    #[serde(rename = "expirationDateTime")]
    pub r#expiration_date_time: Box<String>,
    /// The rate at which a maintenance window is expected to recur.
    #[builder(into)]
    #[serde(rename = "recurEvery")]
    pub r#recur_every: Box<String>,
    /// Effective start date of the maintenance window.
    #[builder(into)]
    #[serde(rename = "startDateTime")]
    pub r#start_date_time: Box<String>,
    /// The time zone for the maintenance window.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
}
