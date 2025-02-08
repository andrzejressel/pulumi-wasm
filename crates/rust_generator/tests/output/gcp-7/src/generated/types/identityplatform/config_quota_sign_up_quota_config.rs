#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigQuotaSignUpQuotaConfig {
    /// A sign up APIs quota that customers can override temporarily. Value can be in between 1 and 1000.
    #[builder(into, default)]
    #[serde(rename = "quota")]
    pub r#quota: Box<Option<i32>>,
    /// How long this quota will be active for. It is measurred in seconds, e.g., Example: "9.615s".
    #[builder(into, default)]
    #[serde(rename = "quotaDuration")]
    pub r#quota_duration: Box<Option<String>>,
    /// When this quota will take affect.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
}
