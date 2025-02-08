#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetUserPoolAccountRecoverySettingRecoveryMechanism {
    /// - Name of the attribute.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// - Priority of this mechanism in the recovery process (lower numbers are higher priority).
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
}
