#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthConfigDecryptedCredentialServiceAccountCredentials {
    /// A space-delimited list of requested scope permissions.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    /// Name of the service account that has the permission to make the request.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
}
