#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiTokenPolicy {
    /// Effect of the policy. Available values: `allow`, `deny`. Defaults to `allow`.
    #[builder(into, default)]
    #[serde(rename = "effect")]
    pub r#effect: Box<Option<String>>,
    /// List of permissions groups IDs. See [documentation](https://developers.cloudflare.com/api/tokens/create/permissions) for more information.
    #[builder(into)]
    #[serde(rename = "permissionGroups")]
    pub r#permission_groups: Box<Vec<String>>,
    /// Describes what operations against which resources are allowed or denied.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Box<std::collections::HashMap<String, String>>,
}
