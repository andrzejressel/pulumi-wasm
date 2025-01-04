#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectEnvironmentTypeUserRoleAssignment {
    /// A list of roles to assign to the `user_id`.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Box<Vec<String>>,
    /// The user object ID that is assigned roles.
    #[builder(into)]
    #[serde(rename = "userId")]
    pub r#user_id: Box<String>,
}
