#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ActiveRoleAssignmentTicket {
    /// User-supplied ticket number to be included with the request. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "number")]
    pub r#number: Box<Option<String>>,
    /// User-supplied ticket system name to be included with the request. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "system")]
    pub r#system: Box<Option<String>>,
}