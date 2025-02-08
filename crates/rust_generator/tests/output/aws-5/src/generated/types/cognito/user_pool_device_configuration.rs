#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPoolDeviceConfiguration {
    /// Whether a challenge is required on a new device. Only applicable to a new device.
    #[builder(into, default)]
    #[serde(rename = "challengeRequiredOnNewDevice")]
    pub r#challenge_required_on_new_device: Box<Option<bool>>,
    /// Whether a device is only remembered on user prompt. `false` equates to "Always" remember, `true` is "User Opt In," and not using a `device_configuration` block is "No."
    #[builder(into, default)]
    #[serde(rename = "deviceOnlyRememberedOnUserPrompt")]
    pub r#device_only_remembered_on_user_prompt: Box<Option<bool>>,
}
