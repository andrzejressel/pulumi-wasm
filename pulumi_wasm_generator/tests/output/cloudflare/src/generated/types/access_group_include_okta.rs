#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessGroupIncludeOkta {
    /// The ID of your Okta identity provider.
    #[builder(into, default)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The name of the Okta Group.
    #[builder(into, default)]
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}
