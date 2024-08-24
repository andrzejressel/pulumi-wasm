#[derive(serde::Serialize)]
pub struct ApiTokenPolicy {
    /// Effect of the policy. Available values: `allow`, `deny`. Defaults to `allow`.
    #[serde(rename = "effect")]
    pub r#effect: Box<Option<String>>,
    /// List of permissions groups IDs. See [documentation](https://developers.cloudflare.com/api/tokens/create/permissions) for more information.
    #[serde(rename = "permissionGroups")]
    pub r#permission_groups: Box<Vec<String>>,
    /// Describes what operations against which resources are allowed or denied.
    #[serde(rename = "resources")]
    pub r#resources: Box<std::collections::HashMap<String, String>>,
}
