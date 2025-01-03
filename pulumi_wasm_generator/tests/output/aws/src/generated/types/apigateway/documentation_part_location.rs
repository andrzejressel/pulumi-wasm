#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DocumentationPartLocation {
    /// HTTP verb of a method. The default value is `*` for any method.
    #[builder(into, default)]
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
    /// Name of the targeted API entity.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// URL path of the target. The default value is `/` for the root resource.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// HTTP status code of a response. The default value is `*` for any status code.
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<String>>,
    /// Type of API entity to which the documentation content appliesE.g., `API`, `METHOD` or `REQUEST_BODY`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
