#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiTokenPolicy {
    /// Effect of the policy. Available values: `allow`, `deny`. Defaults to `allow`.
    #[builder(into, default = Box::new(None))]
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
