#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetUserPoolDeviceConfiguration {
    /// - Whether a challenge is required on new devices.
    #[builder(into)]
    #[serde(rename = "challengeRequiredOnNewDevice")]
    pub r#challenge_required_on_new_device: Box<bool>,
    /// - Whether devices are only remembered if the user prompts it.
    #[builder(into)]
    #[serde(rename = "deviceOnlyRememberedOnUserPrompt")]
    pub r#device_only_remembered_on_user_prompt: Box<bool>,
}
