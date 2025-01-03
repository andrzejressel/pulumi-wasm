#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecurityScanConfigAuthenticationGoogleAccount {
    /// The password of the Google account. The credential is stored encrypted
    /// in GCP.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The user name of the Google account.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
