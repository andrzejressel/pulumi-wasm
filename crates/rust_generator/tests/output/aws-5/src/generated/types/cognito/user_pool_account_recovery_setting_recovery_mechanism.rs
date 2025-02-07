#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserPoolAccountRecoverySettingRecoveryMechanism {
    /// Recovery method for a user. Can be of the following: `verified_email`, `verified_phone_number`, and `admin_only`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Positive integer specifying priority of a method with 1 being the highest priority.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
}
