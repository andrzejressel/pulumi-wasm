#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryGroupRepository {
    /// Required. The Git branch pattern used for indexing in RE2 syntax.
    /// See https://github.com/google/re2/wiki/syntax for syntax.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "branchPattern")]
    pub r#branch_pattern: Box<String>,
    /// Required. The DeveloperConnect repository full resource name, relative resource name
    /// or resource URL to be indexed.
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: Box<String>,
}
