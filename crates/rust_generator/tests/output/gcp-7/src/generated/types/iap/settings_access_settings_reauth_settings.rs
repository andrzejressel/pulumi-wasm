#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SettingsAccessSettingsReauthSettings {
    /// Reauth session lifetime, how long before a user has to reauthenticate again.
    /// A duration in seconds with up to nine fractional digits, ending with 's'.
    /// Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<String>,
    /// Reauth method requested. The possible values are:
    /// * `LOGIN`: Prompts the user to log in again.
    /// * `SECURE_KEY`: User must use their secure key 2nd factor device.
    /// * `ENROLLED_SECOND_FACTORS`: User can use any enabled 2nd factor.
    /// Possible values are: `LOGIN`, `SECURE_KEY`, `ENROLLED_SECOND_FACTORS`.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Box<String>,
    /// How IAP determines the effective policy in cases of hierarchical policies.
    /// Policies are merged from higher in the hierarchy to lower in the hierarchy.
    /// The possible values are:
    /// * `MINIMUM`: This policy acts as a minimum to other policies, lower in the hierarchy.
    /// Effective policy may only be the same or stricter.
    /// * `DEFAULT`: This policy acts as a default if no other reauth policy is set.
    /// Possible values are: `MINIMUM`, `DEFAULT`.
    #[builder(into)]
    #[serde(rename = "policyType")]
    pub r#policy_type: Box<String>,
}
