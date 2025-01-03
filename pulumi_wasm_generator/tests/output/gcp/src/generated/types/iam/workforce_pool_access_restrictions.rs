#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkforcePoolAccessRestrictions {
    /// Services allowed for web sign-in with the workforce pool.
    /// If not set by default there are no restrictions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "allowedServices")]
    pub r#allowed_services: Box<Option<Vec<super::super::types::iam::WorkforcePoolAccessRestrictionsAllowedService>>>,
    /// Disable programmatic sign-in by disabling token issue via the Security Token API endpoint.
    /// See [Security Token Service API](https://cloud.google.com/iam/docs/reference/sts/rest).
    #[builder(into, default)]
    #[serde(rename = "disableProgrammaticSignin")]
    pub r#disable_programmatic_signin: Box<Option<bool>>,
}
